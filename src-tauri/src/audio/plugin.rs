use rodio::Source;
use rodio::source::SamplesConverter;

use super::Sample;
use super::State;

pub enum Command {
	Single(u32),
	Multiple(u32, Vec<Command>),
	Float(f32),
	String(String),
}

impl Command {
	const EMPTY: u32 = 0;
}

/// ## Generator
///
/// Trait for audio generators
///
/// ### Traits
///
/// * `Send` - Can be sent between threads
/// * `Sync` - Is safe to share between threads
///
/// ### Functions
///
/// * `generate(&self, sample_clock: &f32, sample_rate: &f32) -> f32` - Generates a sample
pub trait Generator: Send + Sync {
    fn generate(&mut self, state: &State) -> Sample;
	fn name(&self) -> &'static str;
	fn command(&mut self, command: Command) -> Result<(), String> {
		Err(format!("Command not supported by {}", self.name()))
	}
}

/// ## ClosureGenerator
///
/// A generator that uses a closure to generate samples
///
/// ### Fields
///
/// * `closure: Box<dyn Fn(&f32, &f32) -> f32 + Send + Sync>` - The closure used to generate samples
///
/// ### Examples
///
/// ```
/// let generator = ClosureGenerator::new(Box::new(|sample_clock: &f32, sample_rate: &f32| -> Sample {
/// 	Sample::Mono((sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin())
/// }));
/// ```
pub struct ClosureGenerator {
    closure: Box<dyn Fn(&State) -> Sample + Send + Sync>,
}

impl ClosureGenerator {
    pub fn new(closure: Box<dyn Fn(&State) -> Sample + Send + Sync>) -> Self {
        Self { closure }
    }
}

impl Generator for ClosureGenerator {
    fn generate(&mut self, state: &State) -> Sample {
        (self.closure)(state)
    }

	fn name(&self) -> &'static str {
		"ClosureGenerator"
	}
}

static FALLOFF: f32 = 0.01;

pub struct SineGenerator {
    // First value is frequency, second value is amplitude (0.0-1.0)
    freqs: Vec<(f32, f32)>,
}

impl SineGenerator {
    pub fn new() -> Self {
        Self { freqs: Vec::new() }
    }

    pub fn add_freq(&mut self, freq: f32, amp: f32) {
        self.freqs.push((freq, amp));
    }

    pub fn remove_freq(&mut self, freq: f32) {
        let mut index = 0;
        for (i, freq_amp) in self.freqs.iter().enumerate() {
            if freq_amp.0 == freq {
                index = i;
                break;
            }
        }

        if index >= self.freqs.len() {
            return;
        }

        self.freqs[index].1 = 1.0 - FALLOFF;
    }
}

impl Generator for SineGenerator {
    fn generate(&mut self, state: &State) -> Sample {
        let mut sample = 0.0;
        for freq_amp in self.freqs.iter_mut() {
            if freq_amp.1 < 1.0 {
                freq_amp.1 = freq_amp.1 - FALLOFF;
                if freq_amp.1 < 0.0 {
                    continue;
                }
            }
            sample += (state.sample_clock as f32 * freq_amp.0 * 2.0 * std::f32::consts::PI
                / state.sample_rate as f32)
                .sin()
                * freq_amp.1;
        }

        // remove freqs with amp 0.0
        self.freqs.retain(|freq_amp| freq_amp.1 > 0.0);

        Sample::Stereo(sample, sample)
    }

	fn name(&self) -> &'static str {
		"SineGenerator"
	}
}

pub struct SampleGenerator {
	start: bool,
    stored_clock: u64,
    stored_sample: f32,
    decoder: rodio::Decoder<std::fs::File>,
}

impl SampleGenerator {
    pub fn new(path: &str) -> Self {
        let decoder = rodio::Decoder::new(std::fs::File::open(path).unwrap()).unwrap();
        Self {
			start: false,
            stored_clock: 0,
            stored_sample: 0.0,
            decoder,
        }
    }

	pub const PLAY_SAMPLE: u32 = 1;
	pub const STOP_SAMPLE: u32 = 2;
	pub const SET_SAMPLE: u32 = 3;
}

impl Generator for SampleGenerator {
    fn generate(&mut self, state: &State) -> Sample {
		if !self.start {
			return Sample::Stereo(0.0, 0.0);
		}
        let sample;
        if self.stored_clock < state.sample_clock {
            sample = self.decoder.next().unwrap_or(0) as f32 / 32768.0;
            self.stored_clock = state.sample_clock;
            self.stored_sample = sample;
        } else {
            sample = self.stored_sample;
        }
        Sample::Stereo(sample, sample)
    }

	fn name(&self) -> &'static str {
		"SampleGenerator"
	}

	fn command(&mut self, command: Command) -> Result<(), String> {
		match command {
			Command::Single(command) => {
				match command {
					SampleGenerator::PLAY_SAMPLE => {
						self.start = true;
					}
					SampleGenerator::STOP_SAMPLE => {
						self.start = false;
					}
					_ => {
						return Err(format!("Command {} not supported by {}", command, self.name()));
					}
				}
			}
			Command::Multiple(command, commands) => {
				match command {
					SampleGenerator::SET_SAMPLE => {
						if commands.len() != 1 {
							return Err(format!("Command {} requires 1 argument", command));
						}
						match &commands[0] {
							Command::String(path) => {
								self.decoder = rodio::Decoder::new(std::fs::File::open(path).unwrap()).unwrap();
							}
							_ => {
								return Err(format!("Command {} requires a string argument", command));
							}
						}
					}
					_ => {
						return Err(format!("Command {} not supported by {}", command, self.name()));
					}
				}
			}
			_ => {
				return Err(format!("Command not supported by {}", self.name()));
			}
		}
		Ok(())
	}
}

/// ## Effect
///
/// Trait for audio effects
///
/// ### Traits
///
/// * `Send` - Can be sent between threads
/// * `Sync` - Is safe to share between threads
///
/// ### Functions
///
/// * `process(&mut self, sample: &mut Sample)` - Processes a sample
pub trait Effect: Send + Sync {
    fn process(&mut self, state: &State, sample: &mut Sample);
	fn name(&self) -> &'static str;
}

/// ## Clip
///
/// An effect that clips samples above a certain threshold
///
/// ### Fields
///
/// * `threshold: f32` - The threshold above which samples will be clipped
pub struct Clip {
    threshold: f32,
}

impl Clip {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }
}

impl Effect for Clip {
    fn process(&mut self, _state: &State, sample: &mut Sample) {
        match sample {
            Sample::Mono(sample) => {
                if *sample > self.threshold {
                    *sample = self.threshold;
                } else if *sample < -self.threshold {
                    *sample = -self.threshold;
                }
            }
            Sample::Stereo(left, right) => {
                if *left > self.threshold {
                    *left = self.threshold;
                } else if *left < -self.threshold {
                    *left = -self.threshold;
                }
                if *right > self.threshold {
                    *right = self.threshold;
                } else if *right < -self.threshold {
                    *right = -self.threshold;
                }
            }
        }
    }

	fn name(&self) -> &'static str {
		"Clip"
	}
}

/// ## BitCrusher
///
/// An effect that reduces the bit depth of samples
///
/// ### Fields
///
/// * `bits: u32` - The number of bits to reduce the sample to
pub struct BitCrusher {
    bits: u32,
}

impl BitCrusher {
    pub fn new(bits: u32) -> Self {
        Self { bits }
    }
}

impl Effect for BitCrusher {
    fn process(&mut self, _state: &State, sample: &mut Sample) {
        match sample {
            Sample::Mono(sample) => {
                *sample = (*sample * 2.0f32.powf(self.bits as f32)).floor()
                    / 2.0f32.powf(self.bits as f32);
            }
            Sample::Stereo(left, right) => {
                *left =
                    (*left * 2.0f32.powf(self.bits as f32)).floor() / 2.0f32.powf(self.bits as f32);
                *right = (*right * 2.0f32.powf(self.bits as f32)).floor()
                    / 2.0f32.powf(self.bits as f32);
            }
        }
    }

	fn name(&self) -> &'static str {
		"BitCrusher"
	}
}

/// ## Delay
///
/// An effect that delays samples
///
/// ### Fields
///
/// * `length: usize` - The length of the delay buffer
/// * `feedback: f32` - The amount of feedback to apply to the delay signal
/// * `buffer: Vec<Sample>` - The delay buffer
pub struct Delay {
    length: usize,
    feedback: f32,
    buffer: Vec<Sample>,
}

impl Delay {
    pub fn new(length: usize, feedback: f32) -> Self {
        Self {
            length,
            feedback,
            buffer: vec![Sample::Mono(0.0); length],
        }
    }

    pub fn resize(&mut self, length: usize) {
        self.length = length;
        self.buffer.resize(length, Sample::Mono(0.0));
    }
}

impl Effect for Delay {
    fn process(&mut self, _state: &State, sample: &mut Sample) {
        match sample {
            Sample::Mono(sample) => {
                let delay_signal = self.buffer.remove(0);
                self.buffer
                    .push(Sample::Mono(*sample + delay_signal.mono() * self.feedback));
            }
            Sample::Stereo(left, right) => {
                let delay_signal = self.buffer.remove(0);
                self.buffer.push(Sample::Stereo(
                    *left as f32 + delay_signal.left() * self.feedback,
                    *right as f32 + delay_signal.right() * self.feedback,
                ));
                *left = (*left as f32 + delay_signal.left()) as f32;
                *right = (*right as f32 + delay_signal.right()) as f32;
            }
        }
    }

	fn name(&self) -> &'static str {
		"Delay"
	}
}
