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
	Error
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
	message: String
}

enum ConfigState {
	Saved,
	Unsaved
}

struct Config {
	/// The state of the config
	state: ConfigState,

	/// The path to the config file. This is relative to the config root.
	config_path: String,

	/// The config itself
	config: serde_json::Value
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
		let config = serde_json::json!({
		});
		let config_str = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
		std::fs::write(CONFIG_ROOT.to_owned() + path, config_str).map_err(|e| e.to_string())?;
		return Ok(Config {
			state: ConfigState::Unsaved,
			config_path: path.to_owned(),
			config
		});
	} else {
		// Config exists, load existing config
		debug!("Config exists, loading");
		let config_str = std::fs::read_to_string(CONFIG_ROOT.to_owned() + path).map_err(|e| e.to_string())?;
		let config = serde_json::from_str(&config_str).map_err(|e| e.to_string())?;
		return Ok(Config {
			state: ConfigState::Saved,
			config_path: path.to_owned(),
			config
		});
	}
}

/// Saves the config to the given path
/// 
/// If the config does not exist, it will be created.
fn save_config(config: &Config) -> Result<(), String> {
	debug!("Saving config to {}", config.config_path);
	let config_str = serde_json::to_string_pretty(&config.config).map_err(|e| e.to_string())?;
	std::fs::write(CONFIG_ROOT.to_owned() + &config.config_path, config_str).map_err(|e| e.to_string())?;
	debug!("Saved config");
	Ok(())
}

/// Global config variable based on serde_json
/// This allows it to be saved and loaded from file
static mut GLOBAL_CONFIG: Option<Config> = None;

/// Initializes Rust once the Tauri app is ready
#[tauri::command]
fn tauri_init(window: tauri::Window) -> Result<(), String> {
	debug!("Initializing Tauri");

	// Create the config directory if it does not exist
	std::fs::create_dir_all(CONFIG_ROOT).map_err(|e| e.to_string())?;

	// Load the config
	let w4413_config = load_config("w4113.json")?;

	// Save the config
	save_config(&w4413_config)?;

	// Get default audio config location from the config
	// defaults.audio
	let default_audio_config_location = &w4413_config.config["defaults"]["audio"].as_str();

	match default_audio_config_location {
		Some(location) => {
			debug!("Default audio config location: {}", location);
		},
		None => {
			debug!("No default audio config specified");
		}
	}

	// Set the global config
	unsafe {
		GLOBAL_CONFIG = Some(w4413_config);
	}

	// Make the window visible
	window.show().map_err(|e| e.to_string())?;

	debug!("Initialized Tauri");
	Ok(())
}

fn tauri_call_config(window: tauri::Window, args: Vec<String>) -> ConsoleMessage {
	if args.len() < 1 {
		return ConsoleMessage {
			kind: MessageKind::Error,
			message: "Usage: config [show|load|save]".to_owned()
		};
	} else {
		match args[0].as_str() {
			"show" => {
				// Show the config
				unsafe {
					let config = match &GLOBAL_CONFIG {
						Some(config) => config,
						None => {
							return ConsoleMessage {
								kind: MessageKind::Error,
								message: "Config not loaded".to_owned()
							};
						}
					};
					let message = serde_json::to_string_pretty(&config.config);

					match message {
						Ok(message) => {
							return ConsoleMessage {
								kind: MessageKind::Console,
								message: "Config ".to_owned() + &config.config_path + ": " + &message
							};
						},
						Err(e) => {
							return ConsoleMessage {
								kind: MessageKind::Error,
								message: format!("Error serializing config: {}", e)
							};
						}
					}
				}
			},
			"load" => {
				// TODO load config
				return ConsoleMessage {
					kind: MessageKind::Error,
					message: "Not implemented".to_owned()
				};
			}, 
			"save" => {
				// TODO save config
				return ConsoleMessage {
					kind: MessageKind::Error,
					message: "Not implemented".to_owned()
				};
			},
			_ => {}
		}
	}

	return ConsoleMessage {
		kind: MessageKind::Error,
		message: format!("Usage: config [show|load|save]")
	};
}

#[tauri::command]
fn tauri_call(window: tauri::Window, command: String, args: Vec<String>) -> ConsoleMessage {
	let default_message =  ConsoleMessage {
				kind: MessageKind::Error,
				message: format!("Command not found: {}", command)
			};

	match command.as_str() {
		"config" => {
			return tauri_call_config(window, args);
		},
		_ => {}
	}

	return default_message;
}

fn main() {
    tauri::Builder::default()
		.plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
			tauri_init,
			tauri_call
		])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
