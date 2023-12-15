use log::debug;
use log::trace;
use rodio::source::SamplesConverter;
use rodio::Source;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

use super::Sample;
use super::State;

/// Describes a command that can be sent to an effect or generator
/// 
/// ### Variants
/// * `Single(u32)` - A command with a single argument
/// * `Multiple(u32, Vec<Command>)` - A command with multiple arguments
/// * `Float(f32)` - A command with a single float argument
/// * `String(String)` - A command with a single string argument
#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "../src/bindings/Command.ts")]
pub enum Command {
    Single(u32),
    Multiple(u32, Vec<Command>),
    Float(f32),
    String(String),
}

impl Command {
    const EMPTY: u32 = 0;
}

/// Describes a control that can be sent to an effect or generator
/// 
/// ### Variants
/// * `Dial(String, f32, f32, f32)` - A dial control
/// * `Slider(String, f32, f32, f32)` - A slider control
/// * `Toggle(String, u32, u32)` - A toggle control
/// * `String(String, String)` - A string control
#[derive(TS)]
#[ts(export, export_to = "../src/bindings/Control.ts")]
pub enum Control {
    /// A dial control
    ///
    /// ### Fields
    ///
    /// * `name: String` - The name of the control
    /// * `value: f32` - The value of the control
    /// * `min: f32` - The minimum value of the control
    /// * `max: f32` - The maximum value of the control
    Dial(String, f32, f32, f32),

    /// A slider control
    ///
    /// ### Fields
    ///
    /// * `name: String` - The name of the control
    /// * `value: f32` - The value of the control
    /// * `min: f32` - The minimum value of the control
    /// * `max: f32` - The maximum value of the control
    Slider(String, f32, f32, f32),

    /// A toggle control
    ///
    /// ### Fields
    ///
    /// * `name: String` - The name of the control
    /// * `value: u32` - The value of the control
    /// * `n_states: u32` - The number of states of the control
    Toggle(String, u32, u32),

    /// A string control
    ///
    /// ### Fields
    ///
    /// * `name: String` - The name of the control
    /// * `value: String` - The value of the control
    String(String, String),
}

impl Serialize for Control {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match self {
			Control::Dial(name, value, min, max) => {
				serde_json::json!({
					"kind": "dial",
					"name": name,
					"value": value,
					"min": min,
					"max": max
				}).serialize(serializer)
			}
			Control::Slider(name, value, min, max) => {
				serde_json::json!({
					"kind": "slider",
					"name": name,
					"value": value,
					"min": min,
					"max": max
				}).serialize(serializer)
			}
			Control::Toggle(name, value, n_states) => {
				serde_json::json!({
					"kind": "toggle",
					"name": name,
					"value": value,
					"n_states": n_states
				}).serialize(serializer)
			}
			Control::String(name, value) => {
				serde_json::json!({
					"kind": "string",
					"name": name,
					"value": value
				}).serialize(serializer)
			}
		}
	}
}

impl Control {
    const EMPTY: u32 = 0;

	/// Creates a new dial control
	/// 
	/// ### Arguments
	/// * `name: String` - The name of the control
	/// * `min: f32` - The minimum value of the control
	/// * `max: f32` - The maximum value of the control
    pub fn dial(name: String, min: f32, max: f32) -> Self {
        Self::Dial(name, min, min, max)
    }

	/// Creates a new slider control
	/// 
	/// ### Arguments
	/// * `name: String` - The name of the control
	/// * `min: f32` - The minimum value of the control
	/// * `max: f32` - The maximum value of the control
    pub fn slider(name: String, min: f32, max: f32) -> Self {
        Self::Slider(name, min, min, max)
    }

	/// Creates a new toggle control
	/// 
	/// ### Arguments
	/// * `name: String` - The name of the control
	/// * `n_states: u32` - The number of states of the control
    pub fn toggle(name: String, n_states: u32) -> Self {
        Self::Toggle(name, 0, n_states)
    }

	/// Creates a new string control
	/// 
	/// ### Arguments
	/// * `name: String` - The name of the control
	/// * `value: String` - The value of the control
    pub fn string(name: String, value: String) -> Self {
        Self::String(name, value)
    }
}

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
/// * `name(&self) -> &'static str` - Returns the name of the generator
pub trait Generator: Send + Sync {
	/// Generates a sample
	/// 
	/// ### Arguments
	/// * `state: &State` - The current state of the audio engine
	/// 
	/// ### Returns
	/// * `Sample` - The generated sample
    fn generate(&mut self, state: &State) -> Sample;

	/// Returns the name of the generator
	/// 
	/// ### Returns
	/// * `&'static str` - The name of the generator
    fn name(&self) -> &'static str;

	/// Sends a command to the generator
	/// 
	/// ### Arguments
	/// * `command: Command` - The command to send
	/// 
	/// ### Returns
	/// * `Result<(), String>` - The result of the command
    fn command(&mut self, command: Command) -> Result<(), String> {
        Err(format!("Command not supported by {}", self.name()))
    }

	/// Returns the controls of the generator
	/// 
	/// ### Returns
	/// * `Result<Vec<Control>, String>` - The controls of the generator
    fn controls(&self) -> Result<Vec<Control>, String> {
        Ok(Vec::new())
    }

	/// Sets a control of the generator
	/// 
	/// ### Arguments
	/// * `control: Control` - The control to set
	/// 
	/// ### Returns
	/// * `Result<(), String>` - The result of setting the control
    fn set_control(&mut self, control: Control) -> Result<(), String> {
        Err(format!("Control not supported by {}", self.name()))
    }

	/// Returns the generator as JSON
	/// 
	/// ### Returns
	/// * `serde_json::Value` - The generator as JSON
    fn json(&self) -> serde_json::Value;
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

    fn json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": "ClosureGenerator",
            "controls": []
        })
    }
}

static FALLOFF: f32 = 0.01;

/// A generator that plays a sine wave
/// 
/// ### Fields
/// * `freqs: Vec<(f32, f32)>` - The frequencies and amplitudes of the sine waves
pub struct SineGenerator {
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

    fn json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": "SineGenerator",
            "controls": []
        })
    }
}

/// A generator that plays a sample
/// 
/// ### Fields
/// `start: bool` - Whether the sample should be played
/// `stored_clock: u64` - The last sample clock that was played
/// `stored_sample: f32` - The last sample that was played
/// `decoder: rodio::Decoder<std::fs::File>` - The decoder for the sample
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
            Command::Single(command) => match command {
                SampleGenerator::PLAY_SAMPLE => {
                    self.start = true;
                }
                SampleGenerator::STOP_SAMPLE => {
                    self.start = false;
                }
                _ => {
                    return Err(format!(
                        "Command {} not supported by {}",
                        command,
                        self.name()
                    ));
                }
            },
            Command::Multiple(command, commands) => match command {
                SampleGenerator::SET_SAMPLE => {
                    if commands.len() != 1 {
                        return Err(format!("Command {} requires 1 argument", command));
                    }
                    match &commands[0] {
                        Command::String(path) => {
                            self.decoder =
                                rodio::Decoder::new(std::fs::File::open(path).unwrap()).unwrap();
                        }
                        _ => {
                            return Err(format!("Command {} requires a string argument", command));
                        }
                    }
                }
                _ => {
                    return Err(format!(
                        "Command {} not supported by {}",
                        command,
                        self.name()
                    ));
                }
            },
            _ => {
                return Err(format!("Command not supported by {}", self.name()));
            }
        }
        Ok(())
    }

    fn json(&self) -> serde_json::Value {
		serde_json::json!({
			"name": "SampleGenerator",
			"controls": []
		})
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
/// * `name(&self) -> &'static str` - Returns the name of the effect
/// * `command(&mut self, command: Command) -> Result<(), String>` - Sends a command to the effect
/// * `controls(&self) -> Result<Vec<Control>, String>` - Returns the controls of the effect
/// * `set_control(&mut self, control: Control) -> Result<(), String>` - Sets a control of the effect
/// * `json(&self) -> serde_json::Value` - Returns the effect as JSON
pub trait Effect: Send + Sync {

	/// Processes a sample
	/// 
	/// ### Arguments
	/// * `state: &State` - The current state of the audio engine
	/// * `sample: &mut Sample` - The sample to process
    fn process(&mut self, state: &State, sample: &mut Sample);

	/// Returns the name of the effect
	/// 
	/// ### Returns
	/// * `&'static str` - The name of the effect
    fn name(&self) -> &'static str;

	/// Sends a command to the effect
	/// 
	/// ### Arguments
	/// * `command: Command` - The command to send
	/// 
	/// ### Returns
	/// * `Result<(), String>` - The result of the command
    fn command(&mut self, command: Command) -> Result<(), String> {
        Err(format!("Command not supported by {}", self.name()))
    }

	/// Returns the controls of the effect
	/// 
	/// ### Returns
	/// * `Result<Vec<Control>, String>` - The controls of the effect
    fn controls(&self) -> Result<Vec<Control>, String> {
        Ok(Vec::new())
    }

	/// Sets a control of the effect
	/// 
	/// ### Arguments
	/// * `control: Control` - The control to set
	/// 
	/// ### Returns
	/// * `Result<(), String>` - The result of setting the control
    fn set_control(&mut self, control: Control) -> Result<(), String> {
        Err(format!("Control not supported by {}", self.name()))
    }

	/// Returns the effect as JSON
	/// 
	/// ### Returns
	/// * `serde_json::Value` - The effect as JSON
	fn json(&self) -> serde_json::Value;
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

	fn json(&self) -> serde_json::Value {
		serde_json::json!({
			"name": "Clip",
			"controls": [
				Control::dial("threshold".to_string(), 0.0, 1000.0)
			]
		})
	}

	fn controls(&self) -> Result<Vec<Control>, String> {
		let threshold_control = Control::dial("threshold".to_string(), 0.0, 1.0);
		Ok(vec![threshold_control])
	}

	fn set_control(&mut self, control: Control) -> Result<(), String> {
		match control {
			Control::Dial(_, threshold, _, _) => {
				self.threshold = threshold / 1000.0;
				trace!("[Clip] threshold set to {}", self.threshold);
			}
			_ => {
				return Err(format!("Control not supported by {}", self.name()));
			}
		}
		Ok(())
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

    fn controls(&self) -> Result<Vec<Control>, String> {
        let n_bits_control = Control::slider("bits".to_string(), 1.0, 16.0);
        Ok(vec![n_bits_control])
    }

    fn set_control(&mut self, control: Control) -> Result<(), String> {
        match control {
            Control::Dial(_, bits, _, _) => {
                self.bits = bits as u32;
				trace!("[BitCrusher] bits set to {}", self.bits);
            }
            _ => {
                return Err(format!("Control not supported by {}", self.name()));
            }
        }
        Ok(())
    }

	fn json(&self) -> serde_json::Value {
		serde_json::json!({
			"name": "BitCrusher",
			"controls": [
				Control::slider("bits".to_string(), 1.0, 16.0)
			]
		})
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

	fn controls(&self) -> Result<Vec<Control>, String> {
		let length_control = Control::slider("length".to_string(), 1.0, 100.0);
		let feedback_control = Control::slider("feedback".to_string(), 0.0, 1.0);
		Ok(vec![length_control, feedback_control])
	}

	fn set_control(&mut self, control: Control) -> Result<(), String> {
		match control {
			Control::Slider(name, value, _, _) => {
				match name.as_str() {
					"length" => {
						self.resize(value as usize);
						trace!("[Delay] length set to {}", self.length);
					}
					"feedback" => {
						self.feedback = value / 100.0;
						trace!("[Delay] feedback set to {}", self.feedback);
					}
					_ => {
						return Err(format!("Control not supported by {}", self.name()));
					}
				}
			}
			_ => {
				return Err(format!("Control not supported by {}", self.name()));
			}
		}
		Ok(())
	}

	fn json(&self) -> serde_json::Value {
		serde_json::json!({
			"name": "Delay",
			"controls": [
				Control::slider("length".to_string(), 0.0, 96000.0),
				Control::slider("feedback".to_string(), 0.0, 100.0)
			]
		})
	}
}

/// Gain effect
/// 
/// ### Fields
/// * `gain: f32` - The gain of the effect
pub struct Gain {
	gain: f32,
}

impl Gain {
	pub fn new(gain: f32) -> Self {
		Self {
			gain,
		}
	}
}

impl Effect for Gain {
	fn process(&mut self, _state: &State, sample: &mut Sample) {
		match sample {
			Sample::Mono(sample) => {
				*sample *= self.gain;
			}
			Sample::Stereo(left, right) => {
				*left *= self.gain;
				*right *= self.gain;
			}
		}
	}

	fn name(&self) -> &'static str {
		"Gain"
	}

	fn controls(&self) -> Result<Vec<Control>, String> {
		let gain_control = Control::slider("gain".to_string(), 0.0, 1.0);
		Ok(vec![gain_control])
	}

	fn set_control(&mut self, control: Control) -> Result<(), String> {
		match control {
			Control::Dial(_, gain, _, _) => {
				self.gain = gain / 1000.0;
				trace!("[Gain] gain set to {}", self.gain);
			}
			_ => {
				return Err(format!("Control not supported by {}", self.name()));
			}
		}
		Ok(())
	}

	fn json(&self) -> serde_json::Value {
		serde_json::json!({
			"name": "Gain",
			"controls": [
				Control::slider("gain".to_string(), 0.0, 5000.0)
			]
		})
	}
}