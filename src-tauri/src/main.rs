// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use lazy_static::lazy_static;
use log::{debug, LevelFilter};
use std::{
    fmt::{Display, Formatter},
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::Manager;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};

static CONFIG_FILE: &str = "public/config.json";
static CONFIG_ROOT: &str = "public/config/";

// The current configuration
lazy_static! {
    static ref CONFIG: Mutex<config::Config> = Mutex::new(config::Config::empty());
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

/// ## run(_window: tauri::Window) -> String
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
fn run(window: tauri::Window) -> String {
    let result = match init(window) {
        Ok(()) => "Initialization ran successfully".to_owned(),
        Err(e) => format!("Error in initialization: {}", e),
    };

    // make sure CONFIG_ROOT exists; do nothing if it already exists
    std::fs::create_dir_all(CONFIG_ROOT).unwrap_or_default();

    let config = config::Config::load_from_file(CONFIG_FILE);
    let config_path = match config {
        Ok(mut config) => {
            debug!("Loaded config from {}", CONFIG_FILE);
            match config.get_or("config", || "default.json".to_owned()) {
                Ok(config_path) => {
                    let path = CONFIG_ROOT.to_owned() + &config_path;
                    let _ = config.save_to_file(CONFIG_FILE);
                    path
                }
                Err(e) => {
                    debug!("Error loading config: {}", e);
                    let _ = config.save_to_file(CONFIG_FILE);
                    CONFIG_ROOT.to_owned() + "default.json"
                }
            }
        }
        Err(e) => {
            debug!("Error loading config: {}", e);
            CONFIG_ROOT.to_owned() + "default.json"
        }
    };

    let config = config::Config::load_from_file(&config_path);
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

    let _ = config.save_to_file(config_path.as_str());

    // Set CONFIG to the loaded config
    match CONFIG.lock() {
        Ok(mut config_mutex) => {
            *config_mutex = config;
        }
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
        }
    }

    debug!("{}", result);
    result
}

/// ## init(_window: tauri::Window) -> Result<(), String>
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
fn init(window: tauri::Window) -> Result<(), String> {
    debug!("Initializing Tauri");

    // Make the window visible
    debug!("Calling window.show()");
    window.show().map_err(|e| e.to_string())?;
    Ok(())
}

/// ## config_show() -> ConsoleMessage
///
/// Shows the current config.
///
/// ### Returns
///
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
fn config_show() -> ConsoleMessage {
    let config = CONFIG.lock();
    let config = match config {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking CONFIG: {}", e)],
            };
        }
    };

    let json = config.json().to_string();
    ConsoleMessage {
        kind: MessageKind::Console,
        message: json.lines().map(|s| s.to_owned()).collect(),
    }
}

/// ## config_save(filename: &str) -> ConsoleMessage
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
fn config_save(filename: &str) -> ConsoleMessage {
    let filename = match filename.strip_suffix(".json") {
        Some(filename) => format!("{}.json", filename),
        None => format!("{}.json", filename),
    };
    let filename = format!("{}{}", CONFIG_ROOT, filename);
    let filename = &filename;

    let config = CONFIG.lock();
    let mut config = match config {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking CONFIG: {}", e)],
            };
        }
    };
    let result = config.save_to_file(filename);
    let json = config.json().to_string();
    ConsoleMessage {
        kind: MessageKind::Console,
        message: json.lines().map(|s| s.to_owned()).collect(),
    }
}

/// ## config_load(filename: &str) -> ConsoleMessage
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
fn config_load(filename: &str) -> ConsoleMessage {
    let filename = match filename.strip_suffix(".json") {
        Some(filename) => format!("{}.json", filename),
        None => format!("{}.json", filename),
    };
    let filename = format!("{}{}", CONFIG_ROOT, filename);
    let filename = &filename;

    let config = config::Config::load_from_file(filename);
    let mut config = match config {
        Ok(config) => {
            debug!("Loaded config from {}", filename);
            config
        }
        Err(e) => {
            debug!("Error loading config: {}", e);
            config::Config::empty()
        }
    };

    // Set CONFIG to the loaded config
    match CONFIG.lock() {
        Ok(mut config_mutex) => {
            *config_mutex = config.clone();
        }
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
        }
    }

    let json = config.json().to_string();
    ConsoleMessage {
        kind: MessageKind::Console,
        message: json.lines().map(|s| s.to_owned()).collect(),
    }
}

/// ## host_list(_window: tauri::Window) -> ConsoleMessage
///
/// Lists all available hosts.
///
/// ### Returns
///
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
fn host_list(_window: tauri::Window) -> ConsoleMessage {
    let hosts = audio::list_hosts();

    ConsoleMessage {
        kind: MessageKind::Console,
        message: hosts,
    }
}

/// ## host_select(_window: tauri::Window, host_name: &str) -> ConsoleMessage
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
fn host_select(_window: tauri::Window, host: String) -> ConsoleMessage {
    let host = audio::get_host(&host);
    let host_name = &host.id().name();

    // Set audio::HOST to host
    match audio::HOST.lock() {
        Ok(mut host_mutex) => {
            *host_mutex = Some(host);
        }
        Err(e) => {
            debug!("Error locking HOST: {}", e);
            return ConsoleMessage {
                kind: MessageKind::Error,
                message: vec![format!("Error locking HOST: {}", e)],
            };
        }
    }

    let _ = set_global_config_value("audio.host", host_name);

    return ConsoleMessage {
        kind: MessageKind::Console,
        message: vec![format!("Selected host {}", host_name)],
    };
}

/// ## output_list(_window: tauri::Window) -> ConsoleMessage
/// 
/// Lists all available output devices.
/// 
/// ### Returns
/// 
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
fn output_list(_window: tauri::Window) -> ConsoleMessage {
    let host = audio::HOST.lock();

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
            let devices = audio::list_output_devices(host);
            ConsoleMessage {
                kind: MessageKind::Console,
                message: devices,
            }
        }
        None => ConsoleMessage {
            kind: MessageKind::Error,
            message: vec![format!("No host selected")],
        },
    }
}

/// ## output_select(_window: tauri::Window, device_name: &str) -> ConsoleMessage
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
fn output_select(_window: tauri::Window, output: String) -> ConsoleMessage {
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
					let _ = set_global_config_value("audio.output", device_name);
					ConsoleMessage {
						kind: MessageKind::Console,
						message: vec![format!("Selected output device {}", device_name)],
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

/// ## input_list(_window: tauri::Window) -> ConsoleMessage
/// 
/// Lists all available input devices.
/// 
/// ### Returns
/// 
/// * `ConsoleMessage` - The result of the command
#[tauri::command]
fn input_list(_window: tauri::Window) -> ConsoleMessage {
    let host = audio::HOST.lock();

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
            let devices = audio::list_input_devices(host);
            ConsoleMessage {
                kind: MessageKind::Console,
                message: devices,
            }
        }
        None => ConsoleMessage {
            kind: MessageKind::Error,
            message: vec![format!("No host selected")],
        },
    }
}

/// ## input_select(_window: tauri::Window, device_name: &str) -> ConsoleMessage
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
fn input_select(_window: tauri::Window, input: String) -> ConsoleMessage {
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
					let _ = set_global_config_value("audio.input", device_name);
					ConsoleMessage {
						kind: MessageKind::Console,
						message: vec![format!("Selected input device {}", device_name)],
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

/// ## set_global_config_value(key: &str, value: &str) -> Result<(), String>
/// 
/// Sets a value in the global config.
/// 
/// ### Arguments
/// 
/// * `key: &str` - The key to set
/// * `value: &str` - The value to set
/// 
/// ### Returns
/// 
/// * `Ok(())` - The result of the command
/// * `Err(String)` - The result of the command
fn set_global_config_value(key: &str, value: &str) -> Result<(), String> {
    let mut config = CONFIG.lock();
    let mut config = match config {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return Err(format!("Error locking CONFIG: {}", e));
        }
    };
    config.set(key, value);
    Ok(())
}

/// ## main()
///
/// The main function.
/// This function is called when the program is run. This should not be used to initialize the program, that should be done in `event_loop`.
fn main() {
    tauri::Builder::default()
        .setup(|app| Ok(()))
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout, LogTarget::Webview])
                .with_colors(ColoredLevelConfig::default())
                .level(LevelFilter::Debug)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            run,
            config_show,
            config_save,
            config_load,
            host_list,
            host_select,
            output_list,
			output_select,
            input_list,
			input_select,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
