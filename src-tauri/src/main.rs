// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;

use config::Config;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use lazy_static::lazy_static;
use log::{debug, LevelFilter};
use std::{
    fmt::{Display, Formatter},
    sync::Mutex,
};
use tauri::Manager;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};

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
async fn run(window: tauri::Window) -> String {
    let result = match event_loop(window).await {
        Ok(()) => "Initialization ran successfully".to_owned(),
        Err(e) => format!("Error in initialization: {}", e),
    };

    debug!("{}", result);
    result
}

/// ## event_loop(window: tauri::Window) -> Result<(), String>
///
/// The event loop
///
/// ### Arguments
///
/// * `window: tauri::Window` - The window
///
/// ### Returns
///
/// * `Ok(())` - The event loop ran successfully
/// * `Err(String)` - The error message
async fn event_loop(window: tauri::Window) -> Result<(), String> {
    debug!("Initializing Tauri");
    let mut audio_properties: Option<audio::Properties> = None;

    // Create the config directory if it does not exist
    std::fs::create_dir_all(config::CONFIG_ROOT).map_err(|e| e.to_string())?;

    // The w4113 config ONLY stores the location of the default config
    let config = Config::load("../config.json")?;
    let config_location = &config.json()["config"].as_str();
    let config = match config_location {
        Some(location) => {
            debug!("Config location: {}", location);
            let config = Config::load(location)?;
            audio_properties = Some(audio::Properties::from_config(&config)?);
            Some(config)
        }
        None => {
            debug!("No config location specified");
            audio_properties = Some(audio::Properties::empty());
            None
        }
    };

    debug!("Loading audio properties");

    let mut audio_properties = match audio_properties {
        Some(properties) => properties,
        None => {
            debug!("No audio properties found");
            return Err("Could not load audio properties".to_owned());
        }
    };

    // Make the window visible
    debug!("Calling window.show()");
    window.show().map_err(|e| e.to_string())?;

    // create event loop
    tauri::async_runtime::spawn(async move {
        loop {
            window.listen("audio-test", |event| {
                let message = event.payload().unwrap();
                debug!("Received event: {}", message);
            });

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

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
        .invoke_handler(tauri::generate_handler![run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
