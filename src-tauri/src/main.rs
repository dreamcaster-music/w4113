// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::{Display, Formatter};

use log::debug;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

static CONFIG_ROOT: &str = "target/config";

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

struct AudioProperties {
    /// The audio device to use
    host: Option<cpal::Host>,

    /// The output device to use
    output_device: Option<cpal::Device>,

    /// The input device to use
    input_device: Option<cpal::Device>,

    /// output streams (128 max)
    output_streams: Vec<cpal::Stream>,

    /// input streams (128 max)
    input_streams: Vec<cpal::Stream>,
}

impl AudioProperties {
    fn empty() -> AudioProperties {
        AudioProperties {
            host: None,
            output_device: None,
            input_device: None,
            output_streams: Vec::new(),
            input_streams: Vec::new(),
        }
    }

    fn from_config(config: &Config) -> Result<AudioProperties, String> {
        let host = match config.config["audio"]["host"].as_str() {
            Some(host_name) => {
                let hosts = cpal::available_hosts();
                let mut host: Option<cpal::Host> = None;
                for hostId in hosts {
                    if hostId.name() == host_name {
                        host = Some(cpal::host_from_id(hostId).map_err(|e| e.to_string())?);
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

        let output_device = match config.config["audio"]["output"].as_str() {
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
        let input_device = match config.config["audio"]["input"].as_str() {
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

/// Initializes Rust once the Tauri app is ready
#[tauri::command]
fn event_loop(window: tauri::Window) -> Result<(), String> {
    debug!("Initializing Tauri");
    let mut audio_properties: Option<AudioProperties> = None;

    // Create the config directory if it does not exist
    std::fs::create_dir_all(CONFIG_ROOT).map_err(|e| e.to_string())?;

    // The w4113 config ONLY stores the location of the default config
    let config = load_config("../config.json")?;
    let config_location = &config.config["config"].as_str();
    let config = match config_location {
        Some(location) => {
            debug!("Config location: {}", location);
            let config = load_config(location)?;
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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![event_loop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
