// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;
mod granulizer;
mod interface;
mod midi;
mod tv;

use audio::{plugin::SineGenerator, Preference};
use cpal::traits::DeviceTrait;
use lazy_static::lazy_static;
use log::{debug, error, LevelFilter};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex, RwLock},
};
use tauri::{api::path::BaseDirectory, AppHandle, LogicalPosition, Manager};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};

use crate::{config::Config, interface::Key};

static CONFIG_FILE: &str = "public/config.json";
static CONFIG_ROOT: &str = "public/config/";

// The current configuration
lazy_static! {
    static ref CONFIG: Arc<RwLock<Config>> = Arc::new(RwLock::new(Config::empty()));
    static ref CONSOLE_WINDOW: Mutex<Option<tauri::Window>> = Mutex::new(None);
    static ref TV_WINDOW: Mutex<Option<tauri::Window>> = Mutex::new(None);
}

/// ## MessageKind
///
/// Represents the kind of message that can be sent to the frontend console.
///
/// ### Variants
///
/// * `User` - A message from the user
/// * `Console` - A message from the console
/// * `Error` - An error message
///
/// ### Attributes
///
/// * `#[derive(ts_rs::TS, serde::Serialize, serde::Deserialize)]` - Serde and TS-RS are used to make this enum available to both Rust and TypeScript.
/// * `#[ts(export, export_to = "../src/bindings/MessageKind.ts")]` - This enum is exported to TypeScript, and is used by the frontend console.
#[derive(ts_rs::TS, serde::Serialize, serde::Deserialize)]
#[ts(export, export_to = "../src/bindings/MessageKind.ts")]
enum MessageKind {
    User,
    Console,
    Error,
}

/// ## ConsoleMessage
///
/// Represents a message that can be sent to the frontend console.
///
/// ### Fields
///
/// * `kind: MessageKind` - The kind of message
/// * `message: Vec<String>` - The message itself, split into lines
///
/// ### Attributes
///
/// * `#[derive(ts_rs::TS, serde::Serialize, serde::Deserialize)]` - Serde and TS-RS are used to make this struct available to both Rust and TypeScript.
/// * `#[ts(export, export_to = "../src/bindings/ConsoleMessage.ts")]` - This struct is exported to TypeScript, and is used by the frontend console.
#[derive(ts_rs::TS, serde::Serialize, serde::Deserialize)]
#[ts(export, export_to = "../src/bindings/ConsoleMessage.ts")]
struct ConsoleMessage {
    /// The kind of message
    kind: MessageKind,

    /// The message itself
    message: Vec<String>,
}

/// ## `run(_window: tauri::Window) -> String`
///
/// The run command
///
/// ### Arguments
///
/// * `_window: tauri::Window` - The window
///
/// ### Returns
///
/// * `String` - The result of the command, formatted as a string
#[tauri::command]
async fn run(window: tauri::Window) -> String {
    let result = match init(&window) {
        Ok(()) => "Initialization ran successfully".to_owned(),
        Err(e) => format!("Error in initialization: {}", e),
    };
	debug!("{}", result);

    // make sure CONFIG_ROOT exists; do nothing if it already exists
    std::fs::create_dir_all(CONFIG_ROOT).unwrap_or_default();

    let config = config::Config::load(CONFIG_FILE);
    let config_path = match config {
        Ok(mut config) => {
            debug!("Loaded config from {}", CONFIG_FILE);
            match config.get_or("config", || "default.json".to_owned()) {
                Ok(config_path) => {
                    let path = CONFIG_ROOT.to_owned() + &config_path;
                    let _ = config.save();
                    path
                }
                Err(e) => {
                    debug!("Error loading config: {}", e);
                    let _ = config.save();
                    CONFIG_ROOT.to_owned() + "default.json"
                }
            }
        }
        Err(e) => {
            debug!("Error loading config: {}", e);
            CONFIG_ROOT.to_owned() + "default.json"
        }
    };

    let config = config::Config::load((&config_path));
    let mut config = match config {
        Ok(config) => {
            debug!("Loaded config from {}", config_path);
            config
        }
        Err(e) => {
            debug!("Error loading config: {}", e);
            config::Config::empty()
        }
    };

	config.when_changed("audio.output.device", |key, value| {
				debug!("AUDIO.OUTPUT.DEVICE changed to {}", value);
			});
			config::Config::listen(CONFIG.clone());

    // Set CONFIG to the loaded config
    match CONFIG.write() {
        Ok(mut config_mutex) => {
            *config_mutex = config;
        }
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
        }
    }

    // run audio thread
    let thread_result = audio::audio_thread();
    match thread_result {
        Ok(()) => {
            debug!("Audio thread ran successfully");
        }
        Err(e) => {
            debug!("Error in audio thread: {}", e);
        }
    };

    debug!("{}", result);
    result
}

/// ## `init(_window: tauri::Window) -> Result<(), String>`
///
/// Initializes the program.
///
/// ### Arguments
///
/// * `_window: tauri::Window` - The window
///
/// ### Returns
///
/// * `Result<(), String>` - The result of the command
fn init(window: &tauri::Window) -> Result<(), String> {
    debug!("Initializing Tauri");

    let strips = audio::STRIPS.try_write();

    // let new_strip = audio::Strip::new(
    // 	audio::Input::Generator(Box::new(|sample_clock: &f32, sample_rate: &f32| -> f32 {
    // 		(sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    // 	})),
    // 	audio::Output::Channel(0)
    // );

    // match strips {
    // 	Ok(mut strips) => {
    // 		strips.push(new_strip);
    // 	}
    // 	Err(e) => {
    // 		debug!("Error locking STRIPS: {}", e);
    // 	}
    // }

    let midi_generator = audio::plugin::ClosureGenerator::new(Box::new(midi::callback));
    let mut midi_strip = audio::Strip::new(
        audio::Input::Generator(Arc::new(Mutex::new(midi_generator))),
        audio::Output::Stereo(0, 1),
    );

    midi_strip.add_effect(Box::new(audio::plugin::BitCrusher::new(16)));
    midi_strip.add_effect(Box::new(audio::plugin::Delay::new(
        (44100.0 / 4.0) as usize,
        0.1,
    )));
    //midi_strip.add_effect(Box::new(audio::plugin::LofiDelay::new(500, 0.5, 10)));

    // let mut granulizer = granulizer::Granulizer::new();
    // granulizer.resize_milliseconds(1000, state.sample_rate);
    // midi_strip.add_effect(Box::new(granulizer));

    match strips {
        Ok(mut strips) => {
            strips.push(midi_strip);
        }
        Err(e) => {
            debug!("Error locking STRIPS: {}", e);
        }
    }
    Ok(())
}

/// ## `config_show() -> ConsoleMessage`
///
/// Shows the current config.
///
/// ### Returns
///
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
async fn config_show() -> ConsoleMessage {
    let mut json = "null".to_string();
    match CONFIG.read() {
        Ok(config) => json = config.json().to_string(),
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
        }
    }

    ConsoleMessage {
        kind: MessageKind::Console,
        message: json.lines().map(|s| s.to_owned()).collect(),
    }
}

/// ## `config_save(filename: &str) -> ConsoleMessage`
///
/// Saves the current config to a file.
///
/// ### Arguments
///
/// * `filename: &str` - The name of the file to save to
///
/// ### Returns
///
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
async fn config_save(filename: String) -> ConsoleMessage {
    let filename = match filename.strip_suffix(".json") {
        Some(filename) => format!("{}.json", filename),
        None => format!("{}.json", filename),
    };
    let filename = format!("{}{}", CONFIG_ROOT, filename);
    let filename = &filename;

    let mut config = match CONFIG.write() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking CONFIG: {}", e)],
            };
        }
    };

    let _result = config.save();
    let json = config.json().to_string();
    ConsoleMessage {
        kind: MessageKind::Console,
        message: json.lines().map(|s| s.to_owned()).collect(),
    }
}

/// ## `config_load(filename: &str) -> ConsoleMessage`
///
/// Loads a config from a file.
///
/// ### Arguments
///
/// * `filename: &str` - The name of the file to load from
///
/// ### Returns
///
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
async fn config_load(filename: String) -> ConsoleMessage {
    let filename = match filename.strip_suffix(".json") {
        Some(filename) => format!("{}.json", filename),
        None => format!("{}.json", filename),
    };
    let filename = format!("{}{}", CONFIG_ROOT, filename);
    let filename = &filename;

    let config = config::Config::load(filename);
    let config = match config {
        Ok(config) => {
            debug!("Loaded config from {}", filename);
            config
        }
        Err(e) => {
            debug!("Error loading config: {}", e);
            config::Config::empty()
        }
    };

    ConsoleMessage {
        kind: MessageKind::Console,
        message: vec!["Loaded config".to_string()],
    }
}

/// ## `host_select(_window: tauri::Window, host_name: &str) -> ConsoleMessage`
///
/// Selects a host.
///
/// ### Arguments
///
/// * `host_name: &str` - The name of the host to select
///
/// ### Returns
///
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
async fn host_select(_window: tauri::Window, host: String) -> ConsoleMessage {
    let host = audio::get_host(&host);
    let host_name = &host.id().name();

    // Set audio::HOST to host
    match audio::HOST.lock() {
        Ok(mut host_mutex) => {
            *host_mutex = Some(host);
        }
        Err(e) => {
            error!("Error locking HOST: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking HOST: {}", e)],
            };
        }
    }

    let mut config = match CONFIG.write() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking CONFIG: {}", e)],
            };
        }
    };

    let _ = config.set("audio.host", host_name);

    return ConsoleMessage {
        kind: MessageKind::Console,
        message: vec![format!("Selected host {}", host_name)],
    };
}

/// ## `output_select(_window: tauri::Window, device_name: &str) -> ConsoleMessage`
///
/// Selects an output device.
///
/// ### Arguments
///
/// * `device_name: &str` - The name of the device to select
///
/// ### Returns
///
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
async fn output_select(_window: tauri::Window, output: String) -> ConsoleMessage {
    let host = audio::HOST.lock();
    let device_name = output.as_str();

    let host = match host {
        Ok(host) => host,
        Err(e) => {
            debug!("Error locking HOST: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking HOST: {}", e)],
            };
        }
    };

    match host.as_ref() {
        Some(host) => {
            let device = audio::get_output_device(device_name, host);
            match device {
                Some(device) => {
                    let actual_device_name = match device.name() {
                        Ok(name) => name,
                        Err(e) => {
                            debug!("Error getting device name: {}", e);
                            device_name.to_owned()
                        }
                    };

                    let mut config = match CONFIG.write() {
                        Ok(config) => config,
                        Err(e) => {
                            debug!("Error locking CONFIG: {}", e);
                            return ConsoleMessage {
                                kind: MessageKind::Error,
                                message: vec![format!("Error locking CONFIG: {}", e)],
                            };
                        }
                    };

                    let _ = config.set("audio.output.device", actual_device_name.as_str());
                    ConsoleMessage {
                        kind: MessageKind::Console,
                        message: vec![format!("Selected output device {}", actual_device_name)],
                    }
                }
                None => ConsoleMessage {
                    kind: MessageKind::Error,
                    message: vec![format!("No output device named {}", device_name)],
                },
            }
        }
        None => ConsoleMessage {
            kind: MessageKind::Error,
            message: vec![format!("No host selected")],
        },
    }
}

/// ## `output_stream_set(_window: tauri::Window, channels: u32, samples: u32, buffer_size: u32) -> ConsoleMessage`
///
/// Sets the output stream configuration.
///
/// ### Arguments
///
/// * `channels: u32` - The number of channels
/// * `samples: u32` - The number of samples
/// * `buffer_size: u32` - The buffer size
///
/// ### Returns
///
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
async fn output_stream_set(
    _window: tauri::Window,
    channels: u32,
    samples: u32,
    buffer_size: u32,
) -> ConsoleMessage {
    let output_device = audio::OUTPUT_DEVICE.lock();
    let output_device = match output_device {
        Ok(output_device) => output_device,
        Err(e) => {
            debug!("Error locking OUTPUT_DEVICE: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking OUTPUT_DEVICE: {}", e)],
            };
        }
    };

    let output_device = match output_device.as_ref() {
        Some(output_device) => output_device,
        None => {
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("No output device selected")],
            };
        }
    };

    let config = audio::get_output_config(
        &output_device,
        Preference::Exact(channels as u32, audio::PreferenceAlt::Higher),
        Preference::Exact(samples as u32, audio::PreferenceAlt::Higher),
        Preference::Exact(buffer_size as u32, audio::PreferenceAlt::Higher),
    );

    let result = match &config {
        Some(config) => (
            config.channels as i64,
            config.sample_rate.0 as i64,
            match config.buffer_size {
                cpal::BufferSize::Fixed(buffer_size) => format!("{}", buffer_size),
                cpal::BufferSize::Default => "default".to_owned(),
            },
        ),
        None => (0, 0, "default".to_owned()),
    };

    let channel_result = result.0;
    let sample_result = result.1;
    let buffer_size_result = result.2;

    let inner = CONFIG.write();
    let mut inner = match inner {
        Ok(mut config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking CONFIG: {}", e)],
            };
        }
    };

    let _ = inner.set("audio.output.channels", channel_result.to_string().as_str());
    let _ = inner.set("audio.output.samples", sample_result.to_string().as_str());
    let _ = inner.set("audio.output.buffer_size", buffer_size_result.as_str());

    // set OUTPUT_CONFIG to config
    match audio::OUTPUT_CONFIG.lock() {
        Ok(mut output_config_mutex) => {
            *output_config_mutex = config;
        }
        Err(e) => {
            debug!("Error locking OUTPUT_CONFIG: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking OUTPUT_CONFIG: {}", e)],
            };
        }
    }

    ConsoleMessage {
        kind: MessageKind::Console,
        message: vec![format!(
            "Set output stream to {} channels, {} samples, {} buffer size",
            channel_result, sample_result, buffer_size_result
        )],
    }
}

/// ## `input_select(_window: tauri::Window, device_name: &str) -> ConsoleMessage`
///
/// Selects an input device.
///
/// ### Arguments
///
/// * `device_name: &str` - The name of the device to select
///
/// ### Returns
///
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
async fn input_select(_window: tauri::Window, input: String) -> ConsoleMessage {
    let host = audio::HOST.lock();
    let device_name = input.as_str();

    let host = match host {
        Ok(host) => host,
        Err(e) => {
            debug!("Error locking HOST: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking HOST: {}", e)],
            };
        }
    };

    match host.as_ref() {
        Some(host) => {
            let device = audio::get_input_device(device_name, host);
            match device {
                Some(device) => {
                    let actual_device_name = match device.name() {
                        Ok(name) => name,
                        Err(e) => {
                            debug!("Error getting device name: {}", e);
                            device_name.to_owned()
                        }
                    };

                    let mut config = match CONFIG.write() {
                        Ok(config) => config,
                        Err(e) => {
                            debug!("Error locking CONFIG: {}", e);
                            return ConsoleMessage {
                                kind: MessageKind::Error,
                                message: vec![format!("Error locking CONFIG: {}", e)],
                            };
                        }
                    };

                    let _ = config.set("audio.input.device", actual_device_name.as_str());
                    ConsoleMessage {
                        kind: MessageKind::Console,
                        message: vec![format!("Selected input device {}", actual_device_name)],
                    }
                }
                None => ConsoleMessage {
                    kind: MessageKind::Error,
                    message: vec![format!("No input device named {}", device_name)],
                },
            }
        }
        None => ConsoleMessage {
            kind: MessageKind::Error,
            message: vec![format!("No host selected")],
        },
    }
}

/// ## `input_stream_set(_window: tauri::Window, channels: u32, samples: u32, buffer_size: u32) -> ConsoleMessage`
///
/// Sets the input stream configuration.
///
/// ### Arguments
///
/// * `channels: u32` - The number of channels
/// * `samples: u32` - The number of samples
/// * `buffer_size: u32` - The buffer size
///
/// ### Returns
///
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
async fn input_stream_set(
    _window: tauri::Window,
    channels: u32,
    samples: u32,
    buffer_size: u32,
) -> ConsoleMessage {
    let input_device = audio::INPUT_DEVICE.lock();
    let input_device = match input_device {
        Ok(input_device) => input_device,
        Err(e) => {
            debug!("Error locking INPUT_DEVICE: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking INPUT_DEVICE: {}", e)],
            };
        }
    };

    let input_device = match input_device.as_ref() {
        Some(input_device) => input_device,
        None => {
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("No input device selected")],
            };
        }
    };

    let config = audio::get_input_config(
        &input_device,
        Preference::Exact(channels as u32, audio::PreferenceAlt::Higher),
        Preference::Exact(samples as u32, audio::PreferenceAlt::Higher),
        Preference::Exact(buffer_size as u32, audio::PreferenceAlt::Higher),
    );

    let result = match &config {
        Some(config) => (
            config.channels as i64,
            config.sample_rate.0 as i64,
            match config.buffer_size {
                cpal::BufferSize::Fixed(buffer_size) => format!("{}", buffer_size),
                cpal::BufferSize::Default => "default".to_owned(),
            },
        ),
        None => (0, 0, "default".to_owned()),
    };

    let channel_result = result.0;
    let sample_result = result.1;
    let buffer_size_result = result.2;

    let inner = CONFIG.write();
    let mut inner = match inner {
        Ok(mut config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking CONFIG: {}", e)],
            };
        }
    };

    let _ = inner.set("audio.input.channels", channel_result.to_string().as_str());
    let _ = inner.set("audio.input.samples", sample_result.to_string().as_str());
    let _ = inner.set("audio.input.buffer_size", buffer_size_result.as_str());

    // set INPUT_CONFIG to config
    match audio::INPUT_CONFIG.lock() {
        Ok(mut input_config_mutex) => {
            *input_config_mutex = config;
        }
        Err(e) => {
            debug!("Error locking INPUT_CONFIG: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking INPUT_CONFIG: {}", e)],
            };
        }
    }

    ConsoleMessage {
        kind: MessageKind::Console,
        message: vec![format!(
            "Set input stream to {} channels, {} samples, {} buffer size",
            channel_result, sample_result, buffer_size_result
        )],
    }
}

#[tauri::command]
async fn exit() -> ConsoleMessage {
    match CONFIG.read() {
        Ok(config) => {
            if !config.saved() {
                return ConsoleMessage {
                    kind: MessageKind::Console,
                    message: vec![format!("Unsaved changes. Save before exiting?")],
                };
            }
        }
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
        }
    }

    // otherwise, prompt the user to save
    return ConsoleMessage {
        kind: MessageKind::Console,
        message: vec![format!("Unsaved changes. Save before exiting?")],
    };
}

#[tauri::command]
async fn confirm_exit() -> ConsoleMessage {
    // exit program
    std::process::exit(0);
}

#[tauri::command]
async fn midi_start(_window: tauri::Window, device_name: String) -> ConsoleMessage {
    // let device_name = "AKM320".to_owned();
    // call midi.rs function
    debug!("Calling midi::midi_start()");
    let device_name_clone = device_name.clone();
    let _thread = std::thread::spawn(move || {
        let _midi_devices = midi::midi_start(device_name_clone);
    });
    ConsoleMessage {
        kind: MessageKind::Console,
        message: vec![format!("MIDI device {} started", device_name)],
    }
}

#[tauri::command]
async fn midi_stop(_window: tauri::Window, device_name: String) -> ConsoleMessage {
    // call midi.rs function
    debug!("Calling midi::midi_stop()");
    //let midi_devices = midi::midi_stop(device_name);
    ConsoleMessage {
        kind: MessageKind::Console,
        message: vec![format!("MIDI device {} stopped", device_name)],
    }
}

#[tauri::command]
async fn hid_list(_window: tauri::Window) -> ConsoleMessage {
    // call midi.rs function
    debug!("Calling midi::hid_list()");
    let mut interfaces = interface::get_interfaces();
    for interface in interfaces.iter_mut() {
        if interface.id() == 966156933 {
            let arc_generator = Arc::new(Mutex::new(SineGenerator::new()));
            let mut new_strip = audio::Strip::new(
                audio::Input::Generator(arc_generator.clone()),
                audio::Output::Stereo(0, 1),
            );
            new_strip.add_effect(Box::new(audio::plugin::BitCrusher::new(16)));
            new_strip.add_effect(Box::new(audio::plugin::Delay::new(
                (44100.0 / 4.0) as usize,
                0.1,
            )));

            let arc_clone_keydown = arc_generator.clone();
            let arc_clone_keyup = arc_generator.clone();

            interface.thread();
            interface.keydown(Box::new(move |key| {
                debug!("Key down: {}", key);
                let mut generator = match arc_clone_keydown.lock() {
                    Ok(generator) => generator,
                    Err(err) => {
                        return;
                    }
                };

                let freq: f32 = match key {
                    Key::A => 261.626,
                    Key::W => 277.183,
                    Key::S => 293.665,
                    Key::E => 311.127,
                    Key::D => 329.628,
                    Key::F => 349.228,
                    Key::T => 369.994,
                    Key::G => 391.995,
                    Key::Y => 415.305,
                    Key::H => 440.000,
                    Key::U => 466.164,
                    Key::J => 493.883,
                    Key::K => 523.251,
                    Key::O => 554.365,
                    Key::L => 587.330,
                    Key::P => 622.254,
                    Key::Semicolon => 659.255,
                    Key::Apostrophe => 698.456,
                    _ => 0.0,
                };

                if freq > 0.0 {
                    generator.add_freq(freq, 1.0);
                }
            }));

            interface.keyup(Box::new(move |key| {
                debug!("Key up: {}", key);
                let mut generator = match arc_clone_keyup.lock() {
                    Ok(generator) => generator,
                    Err(err) => {
                        return;
                    }
                };

                let freq: f32 = match key {
                    Key::A => 261.626,
                    Key::W => 277.183,
                    Key::S => 293.665,
                    Key::E => 311.127,
                    Key::D => 329.628,
                    Key::F => 349.228,
                    Key::T => 369.994,
                    Key::G => 391.995,
                    Key::Y => 415.305,
                    Key::H => 440.000,
                    Key::U => 466.164,
                    Key::J => 493.883,
                    Key::K => 523.251,
                    Key::O => 554.365,
                    Key::L => 587.330,
                    Key::P => 622.254,
                    Key::Semicolon => 659.255,
                    Key::Apostrophe => 698.456,
                    _ => 0.0,
                };

                if freq > 0.0 {
                    generator.remove_freq(freq);
                }
            }));

            match audio::STRIPS.write() {
                Ok(mut strips) => {
                    strips.push(new_strip);
                }
                Err(e) => {
                    debug!("Error locking STRIPS: {}", e);
                }
            }
        }
    }
    let mut hid_devices: Vec<String> = Vec::new();
    for interface in &interfaces {
        hid_devices.push(format!("{}", interface));
    }
    ConsoleMessage {
        kind: MessageKind::Console,
        message: hid_devices,
    }
}

/// ## `main()`
///
/// The main function.
/// This function is called when the program is run. This should not be used to initialize the program, that should be done in `event_loop`.
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // set CONSOLE_WINDOW and TV_WINDOW
            let console_window = app.get_window("console").unwrap();
            let tv_window = app.get_window("tv").unwrap();

            let _ = run(console_window.clone());

            match CONSOLE_WINDOW.lock() {
                Ok(mut console_window_mutex) => {
                    *console_window_mutex = Some(console_window);
                }
                Err(e) => {
                    debug!("Error locking CONSOLE_WINDOW: {}", e);
                }
            }

            match TV_WINDOW.lock() {
                Ok(mut tv_window_mutex) => {
                    *tv_window_mutex = Some(tv_window);
                }
                Err(e) => {
                    debug!("Error locking TV_WINDOW: {}", e);
                }
            }

            config::set_app(app.app_handle());

            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout, LogTarget::Webview])
                .level(LevelFilter::Debug)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
			run,
            exit,
            confirm_exit,
            //run,
            config_show,
            config_save,
            config_load,
            audio::list_hosts,
            audio::list_output_devices,
            audio::list_input_devices,
            audio::set_output_device,
            audio::list_output_streams,
            audio::set_input_device,
            audio::list_input_streams,
            host_select,
            output_select,
            output_stream_set,
            input_select,
            input_stream_set,
            midi::midi_list,
            midi_start,
            midi_stop,
            interface::list_interfaces_name,
            hid_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// /// ## `on_config_update(config: &mut config::Config)`
// ///
// /// Called when the config is updated.
// ///
// /// ### Arguments
// ///
// /// * `config: &mut config::Config` - The config
// fn on_config_update(config: &mut config::Config) {
//     debug!("Config updated");
//     let host_name = match config.get_str_or("audio.host", || "default".to_owned()) {
//         Ok(host_name) => host_name,
//         Err(e) => {
//             debug!("Error getting audio.host: {}", e);
//             "default".to_owned()
//         }
//     };
//
//     let host = audio::get_host(&host_name);
//
//     let input_name = match config.get_str_or("audio.input.device", || "default".to_owned()) {
//         Ok(input_name) => input_name,
//         Err(e) => {
//             debug!("Error getting audio.input: {}", e);
//             "default".to_owned()
//         }
//     };
//
//     let output_name = match config.get_str_or("audio.output.device", || "default".to_owned()) {
//         Ok(output_name) => output_name,
//         Err(e) => {
//             debug!("Error getting audio.output: {}", e);
//             "default".to_owned()
//         }
//     };
//
//     let input_channels = match config.get_num_or("audio.input.channels", || 2.0) {
//         Ok(input_channels) => input_channels,
//         Err(e) => {
//             debug!("Error getting audio.input.channels: {}", e);
//             1.0
//         }
//     };
//
//     let input_samples = match config.get_num_or("audio.input.samples", || 44100.0) {
//         Ok(input_samples) => input_samples,
//         Err(e) => {
//             debug!("Error getting audio.input.samples: {}", e);
//             44100.0
//         }
//     };
//
//     let input_buffer_size = match config.get_num_or("audio.input.buffer_size", || 1024.0) {
//         Ok(input_buffer_size) => input_buffer_size,
//         Err(e) => {
//             debug!("Error getting audio.input.buffer_size: {}", e);
//             1024.0
//         }
//     };
//
//     let output_channels = match config.get_num_or("audio.output.channels", || 2.0) {
//         Ok(output_channels) => output_channels,
//         Err(e) => {
//             debug!("Error getting audio.output.channels: {}", e);
//             1.0
//         }
//     };
//
//     let output_samples = match config.get_num_or("audio.output.samples", || 44100.0) {
//         Ok(output_samples) => output_samples,
//         Err(e) => {
//             debug!("Error getting audio.output.samples: {}", e);
//             44100.0
//         }
//     };
//
//     let output_buffer_size = match config.get_num_or("audio.output.buffer_size", || 1024.0) {
//         Ok(output_buffer_size) => output_buffer_size,
//         Err(e) => {
//             debug!("Error getting audio.output.buffer_size: {}", e);
//             1024.0
//         }
//     };
//
//     let input_device = audio::get_input_device(&input_name, &host);
//     let output_device = audio::get_output_device(&output_name, &host);
//
//     match audio::INPUT_DEVICE.try_lock() {
//         Ok(mut input_device_mutex) => {
//             *input_device_mutex = input_device;
//         }
//         Err(e) => {
//             debug!("Error locking INPUT_DEVICE: {}", e);
//         }
//     }
//
//     match audio::OUTPUT_DEVICE.try_lock() {
//         Ok(mut output_device_mutex) => {
//             *output_device_mutex = output_device;
//         }
//         Err(e) => {
//             debug!("Error locking OUTPUT_DEVICE: {}", e);
//         }
//     }
//     let input_device = audio::INPUT_DEVICE.try_lock();
//     match input_device {
//         Ok(input_device) => {
//             if input_device.is_some() {
//                 let input_config;
//                 input_config = audio::get_input_config(
//                     &input_device.as_ref().unwrap(),
//                     Preference::Exact(input_channels as u32, audio::PreferenceAlt::Higher),
//                     Preference::Exact(input_samples as u32, audio::PreferenceAlt::Higher),
//                     Preference::Exact(input_buffer_size as u32, audio::PreferenceAlt::Higher),
//                 );
//
//                 match audio::INPUT_CONFIG.try_lock() {
//                     Ok(mut input_config_mutex) => {
//                         *input_config_mutex = input_config;
//                     }
//                     Err(e) => {
//                         debug!("Error locking INPUT_CONFIG: {}", e);
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             debug!("Error locking INPUT_DEVICE: {}", e);
//             return;
//         }
//     };
//
//     let output_device = audio::OUTPUT_DEVICE.try_lock();
//     match output_device {
//         Ok(output_device) => {
//             if output_device.is_some() {
//                 let output_config;
//                 output_config = audio::get_output_config(
//                     &output_device.as_ref().unwrap(),
//                     Preference::Exact(output_channels as u32, audio::PreferenceAlt::Higher),
//                     Preference::Exact(output_samples as u32, audio::PreferenceAlt::Higher),
//                     Preference::Exact(output_buffer_size as u32, audio::PreferenceAlt::Higher),
//                 );
//
//                 match audio::OUTPUT_CONFIG.try_lock() {
//                     Ok(mut output_config_mutex) => {
//                         *output_config_mutex = output_config;
//                     }
//                     Err(e) => {
//                         debug!("Error locking OUTPUT_CONFIG: {}", e);
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             debug!("Error locking OUTPUT_DEVICE: {}", e);
//             return;
//         }
//     };
//
//     let result = audio::reload();
//     match result {
//         Ok(()) => {
//             debug!("Audio thread ran successfully");
//         }
//         Err(e) => {
//             debug!("Error in audio thread: {}", e);
//         }
//     };
// }
