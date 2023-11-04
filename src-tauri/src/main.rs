// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod audio;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use lazy_static::lazy_static;
use log::{debug, LevelFilter};
use std::{
    fmt::{Display, Formatter},
    sync::{Mutex, Arc}, time::Duration,
};
use tauri::Manager;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};

static CONFIG_FILE: &str = "target/config.json";
static CONFIG_ROOT: &str = "target/config/";

// The current configuration
lazy_static!{
	static ref CONFIG: Mutex<config::Config> = Mutex::new(config::Config::empty());

	static ref HOST: Mutex<Option<cpal::Host>> = Mutex::new(None);
	static ref OUTPUT: Mutex<Option<cpal::Device>> = Mutex::new(None);
	static ref INPUT: Mutex<Option<cpal::Device>> = Mutex::new(None);
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

/// ## run(window: tauri::Window) -> String
///
/// The run command
///
/// ### Arguments
///
/// * `window: tauri::Window` - The window
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
				},
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

    debug!("{}", result);
    result
}

/// ## init(window: tauri::Window) -> Result<(), String>
/// 
/// Initializes the program.
/// 
/// ### Arguments
/// 
/// * `window: tauri::Window` - The window
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
			run
			])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
