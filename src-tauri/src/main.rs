// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::debug;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;
use std::thread;
use std::time::Duration;

#[derive(serde::Serialize, serde::Deserialize)]
enum ConsoleOutputKind {
	User,
	Console,
	Error
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ConsoleOutput {
	kind: ConsoleOutputKind,
	message: String
}

// Global config variable based on serde_json
// This allows it to be saved and loaded from file
static mut CONFIG: Option<serde_json::Value> = None;

#[tauri::command]
fn config_load(path: String) -> ConsoleOutput {
	debug!("Load config from file command invoked");
	let config = match std::fs::read_to_string(path) {
		Ok(config) => config,
		Err(e) => {
			debug!("Error reading config file: {:?}", e);
			return ConsoleOutput {
				kind: ConsoleOutputKind::Error,
				message: format!("Error reading config file: {:?}", e)
			}
		}
	};
	
	let config: serde_json::Value = match serde_json::from_str(&config) {
		Ok(config) => config,
		Err(e) => {
			debug!("Error parsing config file: {:?}", e);
			return ConsoleOutput {
				kind: ConsoleOutputKind::Error,
				message: format!("Error parsing config file: {:?}", e)
			}
		}
	};

	unsafe {
		CONFIG = Some(config.clone());
	}

	ConsoleOutput {
		kind: ConsoleOutputKind::Console,
		message: "Config loaded".to_string()
	}
}

#[tauri::command]
fn config_save(path: String) -> ConsoleOutput {
	debug!("Save config to file command invoked");
	let config = match (unsafe { CONFIG.clone() }) {
		Some(config) => config,
		None => {
			debug!("Config not found, creating default config");
			let config = serde_json::json!({});
			unsafe {
				CONFIG = Some(config.clone());
			}
			config
		}
	};
	let config = match serde_json::to_string_pretty(&config) {
		Ok(config) => config,
		Err(e) => {
			debug!("Error serializing config: {:?}", e);
			return ConsoleOutput {
				kind: ConsoleOutputKind::Error,
				message: format!("Error serializing config: {:?}", e)
			}
		}
	};
	match std::fs::write(path, config) {
		Ok(_) => (),
		Err(e) => {
			debug!("Error writing config file: {:?}", e);
			return ConsoleOutput {
				kind: ConsoleOutputKind::Error,
				message: format!("Error writing config file: {:?}", e)
			}
		}
	};

	ConsoleOutput {
		kind: ConsoleOutputKind::Console,
		message: "Config saved".to_string()
	}
}

#[tauri::command]
fn config_show() -> ConsoleOutput {
	debug!("Show config command invoked");
	let config = match (unsafe { CONFIG.clone() }) {
		Some(config) => config,
		None => {
			debug!("Config not found, creating default config");
			let config = serde_json::json!({});
			unsafe {
				CONFIG = Some(config.clone());
			}
			config
		}
	};
	let config = match serde_json::to_string_pretty(&config) {
		Ok(config) => config,
		Err(e) => {
			debug!("Error serializing config: {:?}", e);
			return ConsoleOutput {
				kind: ConsoleOutputKind::Error,
				message: format!("Error serializing config: {:?}", e)
			}
		}
	};

	ConsoleOutput {
		kind: ConsoleOutputKind::Console,
		message: config
	}
}

#[tauri::command]
fn host_list() -> ConsoleOutput {
	debug!("Host list command invoked");

	// get cpal hosts list
	let hosts = cpal::available_hosts();
	let mut output_hosts: Vec<String> = Vec::new();
	debug!("Available hosts:");
	for host_id in hosts {
		debug!("{:?}", host_id);
		output_hosts.push(host_id.name().to_string());
	}

	ConsoleOutput {
		kind: ConsoleOutputKind::Console,
		message: output_hosts.join("\n")
	}
}

#[tauri::command]
fn host_select(host: String) -> ConsoleOutput {
	debug!("Host select command invoked");

	// add host to config
	let mut config = match (unsafe { CONFIG.clone() }) {
		Some(config) => config,
		None => {
			debug!("Config not found, creating default config");
			let config = serde_json::json!({});
			unsafe {
				CONFIG = Some(config.clone());
			}
			config
		}
	};

	config["host"] = serde_json::Value::String(host.clone());
	unsafe {
		CONFIG = Some(config.clone());
	}

	ConsoleOutput {
		kind: ConsoleOutputKind::Console,
		message: format!("Host set to {}", host)
	}
}

#[tauri::command]
fn device_list() -> ConsoleOutput {
	debug!("Device list command invoked");

	// get hostname from config
	let config = match (unsafe { CONFIG.clone() }) {
		Some(config) => config,
		None => {
			debug!("Config not found, creating default config");
			let config = serde_json::json!({});
			unsafe {
				CONFIG = Some(config.clone());
			}
			config
		}
	};
	let hostname = match config["host"].as_str() {
		Some(hostname) => hostname,
		None => {
			debug!("Host not set");
			return ConsoleOutput {
				kind: ConsoleOutputKind::Error,
				message: "Host not set".to_string()
			}
		}
	};

	// get cpal hosts list
	let hosts = cpal::available_hosts();
	let host_id = hosts
		.iter()
		.find(|h| h.name().to_string() == hostname)
		.unwrap();

	let host = match cpal::host_from_id(*host_id) {
		Ok(host) => host,
		Err(e) => {
			debug!("Error getting host: {:?}", e);
			return ConsoleOutput {
				kind: ConsoleOutputKind::Error,
				message: format!("Error getting host: {:?}", e)
			}
		}
	};

	let devices = match host.devices() {
		Ok(devices) => devices,
		Err(e) => {
			debug!("Error getting devices: {:?}", e);
			return ConsoleOutput {
				kind: ConsoleOutputKind::Error,
				message: format!("Error getting devices: {:?}", e)
			}
		}
	};
	
	let mut output_devices: Vec<String> = Vec::new();
	debug!("Available devices:");
	for device in devices {
		let name = match device.name() {
			Ok(name) => name,
			Err(e) => {
				debug!("Error getting device name: {:?}", e);
				continue;
			}
		};
		debug!("{:?}", name);
		output_devices.push(name);
	}
	
	let mut message = "Available devices:<br>- ".to_string();
	message.push_str(&output_devices.join("<br>- "));

	ConsoleOutput {
		kind: ConsoleOutputKind::Console,
		message
	}
}

#[tauri::command]
fn exit() {
	debug!("Exit command invoked");
	std::process::exit(0);
}

fn main() {
    tauri::Builder::default()
		.plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
			config_load,
			config_save,
			config_show,
			host_select,
			host_list,
			device_list,
			exit,
		])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
