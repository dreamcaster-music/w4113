// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use config::Config;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use log::debug;
use std::fmt::{Display, Formatter};

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

/// ## AudioProperties
/// 
/// Contains all the audio properties needed to manage the audio system.
/// 
/// ### Fields
/// 
/// * `host: Option<cpal::Host>` - The host
/// * `output_device: Option<cpal::Device>` - The output device
/// * `input_device: Option<cpal::Device>` - The input device
/// * `output_streams: Vec<cpal::Stream>` - The output streams -- think of these as the output channels
/// * `input_streams: Vec<cpal::Stream>` - The input streams -- think of these as the input channels
/// 
/// ### Methods
/// 
/// * `empty() -> AudioProperties` - Create an empty audio properties struct
/// * `from_config(config: &Config) -> Result<AudioProperties, String>` - Create an audio properties struct from the given config
struct AudioProperties {
    host: Option<cpal::Host>,
    output_device: Option<cpal::Device>,
    input_device: Option<cpal::Device>,
    output_streams: Vec<cpal::Stream>,
    input_streams: Vec<cpal::Stream>,
}

impl AudioProperties {
	/// ## empty() -> AudioProperties
	/// 
	/// Create an empty audio properties struct
	/// 
	/// ### Returns
	/// 
	/// * `AudioProperties` - The empty audio properties struct
    fn empty() -> AudioProperties {
        AudioProperties {
            host: None,
            output_device: None,
            input_device: None,
            output_streams: Vec::new(),
            input_streams: Vec::new(),
        }
    }

	/// ## from_config(config: &Config) -> Result<AudioProperties, String>
	/// 
	/// Create an audio properties struct from the given config
	/// 
	/// ### Arguments
	/// 
	/// * `config: &Config` - The config
	/// 
	/// ### Returns
	/// 
	/// * `Ok(AudioProperties)` - The audio properties struct
	/// * `Err(String)` - The error message
    fn from_config(config: &Config) -> Result<AudioProperties, String> {
        let host = match config.json()["audio"]["host"].as_str() {
            Some(host_name) => {
                let hosts = cpal::available_hosts();
                let mut host: Option<cpal::Host> = None;
                for host_id in hosts {
                    if host_id.name() == host_name {
                        host = Some(cpal::host_from_id(host_id).map_err(|e| e.to_string())?);
                        break;
                    }
                }
                host
            }
            None => None,
        };

        let host = match host {
            Some(host) => host,
            None => {
                let host = cpal::default_host();
                let name = host.id().name();
                debug!("Using default host {}", name);
                host
            }
        };

        let output_device = match config.json()["audio"]["output"].as_str() {
            Some(device_name) => {
                let device = host
                    .output_devices()
                    .map_err(|e| e.to_string())?
                    .find(|device| {
                        device
                            .name()
                            .map(|name| name == device_name)
                            .unwrap_or(false)
                    })
                    .ok_or(format!("Could not find output device {}", device_name))?;
                Some(device)
            }
            None => None,
        };
        let input_device = match config.json()["audio"]["input"].as_str() {
            Some(device_name) => {
                let device = host
                    .input_devices()
                    .map_err(|e| e.to_string())?
                    .find(|device| {
                        device
                            .name()
                            .map(|name| name == device_name)
                            .unwrap_or(false)
                    })
                    .ok_or(format!("Could not find input device {}", device_name))?;
                Some(device)
            }
            None => None,
        };

        let output_streams = Vec::new();
        let input_streams = Vec::new();

        Ok(AudioProperties {
            host: Some(host),
            output_device,
            input_device,
            output_streams,
            input_streams,
        })
    }
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
/// 
/// ### Attributes
/// 
/// * `#[tauri::command]` - This function is exposed to the frontend
#[tauri::command]
fn event_loop(window: tauri::Window) -> Result<(), String> {
    debug!("Initializing Tauri");
    let mut audio_properties: Option<AudioProperties> = None;

    // Create the config directory if it does not exist
    std::fs::create_dir_all(config::CONFIG_ROOT).map_err(|e| e.to_string())?;

    // The w4113 config ONLY stores the location of the default config
    let config = Config::load("../config.json")?;
    let config_location = &config.json()["config"].as_str();
    let config = match config_location {
        Some(location) => {
            debug!("Config location: {}", location);
            let config = Config::load(location)?;
            audio_properties = Some(AudioProperties::from_config(&config)?);
            Some(config)
        }
        None => {
            debug!("No config location specified");
            audio_properties = Some(AudioProperties::empty());
            None
        }
    };

    let mut audio_properties = match audio_properties {
        Some(properties) => properties,
        None => return Err("Could not load audio properties".to_owned()),
    };

    // Make the window visible
    window.show().map_err(|e| e.to_string())?;

    // Create the event loop
    std::thread::spawn(move || loop {});

    Ok(())
}

/// ## main()
/// 
/// The main function.
/// This function is called when the program is run. This should not be used to initialize the program, that should be done in `event_loop`.
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![event_loop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
