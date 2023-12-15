use std::sync::RwLock;

use log::debug;

use crate::audio::*;

lazy_static::lazy_static! {
	pub static ref RELOAD: RwLock<bool> = RwLock::new(false);
}

/// ## `reload() -> Result<(), String>`
///
/// Reloads the audio thread.
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
pub fn reload() {
    let mut reload = match RELOAD.write() {
        Ok(reload) => reload,
        Err(e) => {
            debug!("Error locking RELOAD: {}", e);
            return;
        }
    };

    *reload = true;
}
/// ## `audio_thread() -> Result<(), String>`
///
/// Starts the audio thread.
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
pub fn run() -> Result<(), String> {
    // emit event to indicate that the audio thread is starting
    crate::try_emit("updatethread", true);

    let thread = std::thread::spawn(move || {
        let config = {
            match OUTPUT_CONFIG.lock() {
                Ok(config) => match config.as_ref() {
                    Some(config) => config.clone(),
                    None => {
                        debug!("OUTPUT_CONFIG is None");
                        //return Err(format!("OUTPUT_CONFIG is None"));

                        // specify type of Err to avoid type mismatch

                        crate::try_emit("updatethread", false);
                        return Err("OUTPUT_CONFIG is None".to_owned());
                    }
                },
                Err(e) => {
                    debug!("Error locking OUTPUT_CONFIG: {}", e);

                    crate::try_emit("updatethread", false);
                    return Err(format!("Error locking OUTPUT_CONFIG: {}", e));
                }
            }
        };

        let output_stream_opt: Option<Result<cpal::Stream, cpal::BuildStreamError>>;

        {
            let output_device = OUTPUT_DEVICE.lock();
            let output_device = match output_device {
                Ok(output_device) => output_device,
                Err(e) => {
                    debug!("Error locking OUTPUT_DEVICE: {}", e);
                    crate::try_emit("updatethread", false);
                    return Err(format!("Error locking OUTPUT_DEVICE: {}", e));
                }
            };

            let output_device = match output_device.as_ref() {
                Some(output_device) => output_device,
                None => {
                    debug!("OUTPUT_DEVICE is None");
                    crate::try_emit("updatethread", false);
                    return Err("OUTPUT_DEVICE is None".to_owned());
                }
            };

            // Produce a sinusoid of maximum amplitude.
            let mut sample_clock = 0f32;

            let n_channels = config.channels as u32;

            let data_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let buffer_size = data.len();
                let mut strips = match STRIPS.try_write() {
                    Ok(strips) => strips,
                    Err(e) => {
                        debug!("Error locking STRIPS: {}", e);
                        crate::try_emit("updatethread", false);
                        return;
                    }
                };

                let mut channel = 0;

                // cpal audio is interleaved, meaning that every sample is followed by another sample for the next channel
                // example: in a stereo stream, the first sample is for the left channel, the second sample is for the right channel, the third sample is for the left channel, etc.
                // So every other sample is for the same channel
                //
                // So there is a simple formula for determining what channel a sample is for:
                // channel = sample_index % n_channels
                let mut data_vec = Vec::new();
                for sample in data.iter_mut() {
                    if channel % n_channels == 0 {
                        sample_clock += 1.0;
                    }

                    for strip in strips.iter_mut() {
                        match strip.output {
                            Output::Mono(strip_channel) => {
                                if strip_channel == channel % n_channels {
                                    *sample = strip
                                        .process(State {
                                            sample_rate: config.sample_rate.0 as u32,
                                            sample_clock: sample_clock as u64,
                                            buffer_size,
                                        })
                                        .mono();
                                }
                            }
                            Output::Stereo(left_channel, right_channel) => {
                                if left_channel == channel % n_channels {
                                    *sample = strip
                                        .process(State {
                                            sample_rate: config.sample_rate.0 as u32,
                                            sample_clock: sample_clock as u64,
                                            buffer_size,
                                        })
                                        .left();
                                } else if right_channel == channel % n_channels {
                                    *sample = strip
                                        .process(State {
                                            sample_rate: config.sample_rate.0 as u32,
                                            sample_clock: sample_clock as u64,
                                            buffer_size,
                                        })
                                        .right();
                                }
                            }
                            _ => {}
                        }
                    }

                    if channel % n_channels == 0 {
                        data_vec.push(*sample);
                    }
                    channel += 1;
                }

                let tv_window = crate::TV_WINDOW.lock();
                match tv_window {
                    Ok(tv_window) => match tv_window.as_ref() {
                        Some(tv_window) => {
                            let visualizer = <BasicVisualizer as VisualizerTrait>::new();
                            let _ = visualizer.render(tv_window, &data_vec);
                        }
                        None => {
                            debug!("TV_WINDOW is None");
                        }
                    },
                    Err(e) => {
                        debug!("Error locking TV_WINDOW: {}", e);
                    }
                }
            };

            let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
            let output_stream =
                output_device.build_output_stream(&config, data_callback, err_fn, None);
            output_stream_opt = Some(output_stream);
        }

        let output_stream = match output_stream_opt {
            Some(output_stream) => output_stream,
            None => {
                crate::try_emit("updatethread", false);
                return Err("Error building output stream".to_owned());
            }
        };

        let output_stream = match output_stream {
            Ok(stream) => stream,
            Err(err) => {
                crate::try_emit("updatethread", false);
                return Err(format!("Error building output stream: {}", err));
            }
        };

        let _ = output_stream.play();

        let mut reload = false;
        while (!reload) {
            std::thread::sleep(std::time::Duration::from_millis(1000));

            match RELOAD.try_write() {
                Ok(mut r) => {
                    if *r {
                        reload = true;
                        *r = false;
                    }
                }
                Err(_err) => {}
            }
        }

        let _ = output_stream.pause();

        crate::try_emit("updatethread", false);
        let new_thread = run();
        debug!("Reloading audio thread... {:?}", new_thread);
        Ok(())
    });

    Ok(())
}