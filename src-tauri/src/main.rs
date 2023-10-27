// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::debug;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// The kind of message that can be used by the frontend. This is also used on the TypeScript side for its own functions.
///
/// Serde and TS-RS are used to make this enum available to both Rust and TypeScript.
#[derive(ts_rs::TS, serde::Serialize, serde::Deserialize)]
#[ts(export, export_to = "../src/bindings/MessageKind.ts")]
enum MessageKind {
    /// A message from the user
    User,

    /// A message from the console
    Console,

    /// An error message
    Error,
}

/// A message that can be sent to the frontend
///
/// This exists so that we can send messages to React and have some extra data along with it. React needs the information of
/// what kind of message it is so that it can be displayed correctly.
///
/// Serde and TS-RS are used to make this struct available to both Rust and TypeScript.
#[derive(ts_rs::TS, serde::Serialize, serde::Deserialize)]
#[ts(export, export_to = "../src/bindings/ConsoleMessage.ts")]
struct ConsoleMessage {
    /// The kind of message
    kind: MessageKind,

    /// The message itself
    message: String,
}

enum ConfigState {
    Saved,
    Unsaved,
}

struct Config {
    /// The state of the config
    state: ConfigState,

    /// The path to the config file. This is relative to the config root.
    config_path: String,

    /// The config itself
    config: serde_json::Value,
}

static CONFIG_ROOT: &str = "target/config/";

/// Loads the config from the given path
///
/// If the config does not exist, it will be created.
fn load_config(path: &str) -> Result<Config, String> {
    debug!("Loading config from {}", path);

    if !std::path::Path::new(&(CONFIG_ROOT.to_owned() + path)).exists() {
        // Config does not exist, create the config
        debug!("Config does not exist, creating");
        let config = serde_json::json!({});
        let config_str = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
        std::fs::write(CONFIG_ROOT.to_owned() + path, config_str).map_err(|e| e.to_string())?;
        return Ok(Config {
            state: ConfigState::Unsaved,
            config_path: path.to_owned(),
            config,
        });
    } else {
        // Config exists, load existing config
        debug!("Config exists, loading");
        let config_str =
            std::fs::read_to_string(CONFIG_ROOT.to_owned() + path).map_err(|e| e.to_string())?;
        let config = serde_json::from_str(&config_str).map_err(|e| e.to_string())?;
        return Ok(Config {
            state: ConfigState::Saved,
            config_path: path.to_owned(),
            config,
        });
    }
}

/// Saves the config to the given path
///
/// If the config does not exist, it will be created.
fn save_config(config: &Config) -> Result<(), String> {
    debug!("Saving config to {}", config.config_path);
    let config_str = serde_json::to_string_pretty(&config.config).map_err(|e| e.to_string())?;
    std::fs::write(CONFIG_ROOT.to_owned() + &config.config_path, config_str)
        .map_err(|e| e.to_string())?;
    debug!("Saved config");
    Ok(())
}

/// Global config variable based on serde_json
/// This allows it to be saved and loaded from file
static mut CONFIG: Option<Config> = None;

/// Initializes Rust once the Tauri app is ready
#[tauri::command]
fn tauri_init(window: tauri::Window) -> Result<(), String> {
    debug!("Initializing Tauri");

    // Create the config directory if it does not exist
    std::fs::create_dir_all(CONFIG_ROOT).map_err(|e| e.to_string())?;

    // The w4113 config ONLY stores the location of the default config
    let w4113_config = load_config("../config.json")?;

    // Get default audio config location from the config
    // defaults.audio
    let default_audio_config_location = &w4113_config.config["config"].as_str();

    unsafe {
        CONFIG = match default_audio_config_location {
            Some(location) => {
                debug!("Default audio config location: {}", location);
                Some(load_config(location)?)
            }
            None => {
                debug!("No default audio config specified");
                Some(load_config("default.json")?)
            }
        }
    }

    // Make the window visible
    window.show().map_err(|e| e.to_string())?;

    debug!("Initialized Tauri");
    Ok(())
}

fn tauri_call_config(window: tauri::Window, args: Vec<String>) -> ConsoleMessage {
    let default_message = ConsoleMessage {
        kind: MessageKind::Error,
        message: "Usage: config [show|default|load|save]".to_owned(),
    };

    if args.len() < 1 {
        return default_message;
    } else {
        match args[0].as_str() {
            "show" => {
                let config = unsafe { CONFIG.as_ref().unwrap() };
                let config_str =
                    serde_json::to_string_pretty(&config.config).map_err(|e| e.to_string());
                let config_str = match config_str {
                    Ok(config_str) => config_str,
                    Err(e) => {
                        return ConsoleMessage {
                            kind: MessageKind::Error,
                            message: e,
                        };
                    }
                };
                return ConsoleMessage {
                    kind: MessageKind::Console,
                    message: config_str,
                };
            }
            "default" => {
                // Set default config
                let config = unsafe { CONFIG.as_mut().unwrap() };

                let w4113_config = load_config("../config.json");
                let mut w4113_config = match w4113_config {
                    Ok(w4113_config) => w4113_config,
                    Err(e) => {
                        return ConsoleMessage {
                            kind: MessageKind::Error,
                            message: e,
                        };
                    }
                };

                w4113_config.config["config"] =
                    serde_json::Value::String(config.config_path.to_owned());
                let save_result = save_config(&w4113_config);

                match save_result {
                    Ok(_) => {
                        return ConsoleMessage {
                            kind: MessageKind::Console,
                            message: format!("Default config set to {}", config.config_path),
                        };
                    }
                    Err(e) => {
                        return ConsoleMessage {
                            kind: MessageKind::Error,
                            message: e,
                        };
                    }
                }
            }
            "load" => {
                if args.len() < 2 {
                    return default_message;
                } else {
                    let filename = match args[1].ends_with(".json") {
                        true => args[1].to_owned(),
                        false => format!("{}.json", args[1]),
                    };

                    let config_path = &filename;
                    let config = unsafe { CONFIG.as_mut().unwrap() };
                    let new_config = load_config(config_path);
                    let new_config = match new_config {
                        Ok(new_config) => new_config,
                        Err(e) => {
                            return ConsoleMessage {
                                kind: MessageKind::Error,
                                message: e,
                            };
                        }
                    };
                    config.config = new_config.config;
                    config.config_path = new_config.config_path;
                    config.state = new_config.state;
                    return ConsoleMessage {
                        kind: MessageKind::Console,
                        message: format!("Loaded config {}", config_path),
                    };
                }
            }
            "save" => {
                if args.len() < 2 {
                    return default_message;
                } else {
                    let filename = match args[1].ends_with(".json") {
                        true => args[1].to_owned(),
                        false => format!("{}.json", args[1]),
                    };

                    let config_path = &filename;
                    let config = unsafe { CONFIG.as_mut().unwrap() };
                    config.config_path = config_path.to_owned();
                    let save_result = save_config(config);
                    match save_result {
                        Ok(_) => {
                            return ConsoleMessage {
                                kind: MessageKind::Console,
                                message: format!("Saved config {}", config_path),
                            };
                        }
                        Err(e) => {
                            return ConsoleMessage {
                                kind: MessageKind::Error,
                                message: e,
                            };
                        }
                    }
                }
            }
            _ => {}
        }
    }

    return default_message;
}

fn tauri_call_host(window: tauri::Window, args: Vec<String>) -> ConsoleMessage {
    let default_message = ConsoleMessage {
        kind: MessageKind::Error,
        message: "Usage: host [list|select|clear] <hostname>".to_owned(),
    };

    if args.len() < 1 {
        return default_message;
    } else {
        match args[0].as_str() {
            "list" => {
                let hosts = cpal::available_hosts();
                let mut message = String::new();
                for host in hosts {
                    message.push_str(&format!("{}\n", host.name()));
                }
                return ConsoleMessage {
                    kind: MessageKind::Console,
                    message,
                };
            }
            "clear" => {
                // Store host_name in CONFIG as audio.host
                unsafe {
                    CONFIG.as_mut().unwrap().config["audio"]["host"] = serde_json::Value::Null;
                }

                return ConsoleMessage {
                    kind: MessageKind::Console,
                    message: "Cleared host".to_owned(),
                };
            }
            "select" => {
                if args.len() < 2 {
                    return default_message;
                } else {
                    let host_name = &args[1];

                    let hosts = cpal::available_hosts();
                    let host = hosts.iter().find(|h| h.name() == host_name);
                    let host = match host {
                        Some(host) => host,
                        None => {
                            return ConsoleMessage {
                                kind: MessageKind::Error,
                                message: format!("Host not found: {}", host_name),
                            };
                        }
                    };

                    let host_name = host.name();
                    let mut message = String::new();
                    message.push_str(&format!("Selected host: {}", &host_name));

                    // Store host_name in CONFIG as audio.host
                    unsafe {
                        CONFIG.as_mut().unwrap().config["audio"]["host"] =
                            serde_json::Value::String(host_name.to_owned());
                    }

                    return ConsoleMessage {
                        kind: MessageKind::Console,
                        message,
                    };
                }
            }
            _ => {}
        }

        return default_message;
    }
}

fn tauri_call_output(window: tauri::Window, args: Vec<String>) -> ConsoleMessage {
    let default_message = ConsoleMessage {
        kind: MessageKind::Error,
        message: "Usage: output [list|select] <device name>".to_owned(),
    };

    if args.len() < 1 {
        return default_message;
    } else {
        // Get host from config
        let host_name = unsafe { CONFIG.as_ref().unwrap().config["audio"]["host"].as_str() };
        let host_name = match host_name {
            Some(host_name) => host_name,
            None => {
                return ConsoleMessage {
                    kind: MessageKind::Error,
                    message: "No host selected".to_owned(),
                };
            }
        };

        // Get host
        let hosts = cpal::available_hosts();
        let host = hosts.iter().find(|h| h.name() == host_name);
        let host = match host {
            Some(host) => host,
            None => {
                return ConsoleMessage {
                    kind: MessageKind::Error,
                    message: format!("Host not found: {}", host_name),
                };
            }
        };

        // Get host from hostID
        let host = match cpal::host_from_id(*host) {
            Ok(host) => host,
            Err(e) => {
                return ConsoleMessage {
                    kind: MessageKind::Error,
                    message: format!("Error getting host: {}", e),
                };
            }
        };

        // Get devices
        let devices = match host.output_devices() {
            Ok(devices) => devices,
            Err(e) => {
                return ConsoleMessage {
                    kind: MessageKind::Error,
                    message: format!("Error getting devices: {}", e),
                };
            }
        };

        match args[0].as_str() {
            "list" => {
                let mut message = String::new();
                for device in devices {
                    let device_name = match device.name() {
                        Ok(device_name) => device_name,
                        Err(e) => {
                            return ConsoleMessage {
                                kind: MessageKind::Error,
                                message: format!("Error getting device name: {}", e),
                            };
                        }
                    };
                    message.push_str(&format!("\"{}\", ", device_name));
                }

                message = message.trim_end_matches(", ").to_owned();

                return ConsoleMessage {
                    kind: MessageKind::Console,
                    message,
                };
            }
            "select" => {
                if args.len() < 2 {
                    return default_message;
                } else {
                    for device in devices {
						let device_name = match device.name() {
							Ok(device_name) => device_name,
							Err(e) => {
								return ConsoleMessage {
									kind: MessageKind::Error,
									message: format!("Error getting device name: {}", e),
								};
							}
						};
						
						if device_name == args[1] {
							// Store device_name in CONFIG as audio.output
							unsafe {
								CONFIG.as_mut().unwrap().config["audio"]["output"] =
									serde_json::Value::String(device_name.to_owned());
							}

							return ConsoleMessage {
								kind: MessageKind::Console,
								message: format!("Selected output: {}", device_name),
							};
						}
					}

					return ConsoleMessage {
						kind: MessageKind::Error,
						message: format!("Output not found: {}", args[1]),
					};
                }
            }
            _ => {}
        }
    }

    return default_message;
}

fn tauri_call_input(window: tauri::Window, args: Vec<String>) -> ConsoleMessage {
    let default_message = ConsoleMessage {
        kind: MessageKind::Error,
        message: "Usage: input [list|select] <device name>".to_owned(),
    };

    if args.len() < 1 {
        return default_message;
    } else {
        // Get host from config
        let host_name = unsafe { CONFIG.as_ref().unwrap().config["audio"]["host"].as_str() };
        let host_name = match host_name {
            Some(host_name) => host_name,
            None => {
                return ConsoleMessage {
                    kind: MessageKind::Error,
                    message: "No host selected".to_owned(),
                };
            }
        };

        // Get host
        let hosts = cpal::available_hosts();
        let host = hosts.iter().find(|h| h.name() == host_name);
        let host = match host {
            Some(host) => host,
            None => {
                return ConsoleMessage {
                    kind: MessageKind::Error,
                    message: format!("Host not found: {}", host_name),
                };
            }
        };

        // Get host from hostID
        let host = match cpal::host_from_id(*host) {
            Ok(host) => host,
            Err(e) => {
                return ConsoleMessage {
                    kind: MessageKind::Error,
                    message: format!("Error getting host: {}", e),
                };
            }
        };

        // Get devices
        let devices = match host.input_devices() {
            Ok(devices) => devices,
            Err(e) => {
                return ConsoleMessage {
                    kind: MessageKind::Error,
                    message: format!("Error getting devices: {}", e),
                };
            }
        };

        match args[0].as_str() {
            "list" => {
                let mut message = String::new();
                for device in devices {
                    let device_name = match device.name() {
                        Ok(device_name) => device_name,
                        Err(e) => {
                            return ConsoleMessage {
                                kind: MessageKind::Error,
                                message: format!("Error getting device name: {}", e),
                            };
                        }
                    };
                    message.push_str(&format!("\"{}\", ", device_name));
                }

                message = message.trim_end_matches(", ").to_owned();

                return ConsoleMessage {
                    kind: MessageKind::Console,
                    message,
                };
            }
            "select" => {
                if args.len() < 2 {
                    return default_message;
                } else {
					for device in devices {
						let device_name = match device.name() {
							Ok(device_name) => device_name,
							Err(e) => {
								return ConsoleMessage {
									kind: MessageKind::Error,
									message: format!("Error getting device name: {}", e),
								};
							}
						};
						
						if device_name == args[1] {
							// Store device_name in CONFIG as audio.output
							unsafe {
								CONFIG.as_mut().unwrap().config["audio"]["input"] =
									serde_json::Value::String(device_name.to_owned());
							}

							return ConsoleMessage {
								kind: MessageKind::Console,
								message: format!("Selected input: {}", device_name),
							};
						}
					}

					return ConsoleMessage {
						kind: MessageKind::Error,
						message: format!("input not found: {}", args[1]),
					};
                }
            }
            _ => {}
        }
    }

    return default_message;
}

#[tauri::command]
fn tauri_call(window: tauri::Window, command: String, args: Vec<String>) -> ConsoleMessage {
    debug!("tauri_call\ncommand:{}\nargs:{:?}", command, args);

    let default_message = ConsoleMessage {
        kind: MessageKind::Error,
        message: format!("Command not found: {}", command),
    };

    match command.as_str() {
        "config" => {
            return tauri_call_config(window, args);
        }
        "host" => {
            return tauri_call_host(window, args);
        }
        "output" => {
            return tauri_call_output(window, args);
        }
        "input" => {
            return tauri_call_input(window, args);
        }
        _ => {}
    }

    return default_message;
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![tauri_init, tauri_call])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
