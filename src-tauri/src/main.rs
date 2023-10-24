// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::debug;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
	use std::f32::consts::PI;
	use std::thread;
	use std::time::Duration;

#[tauri::command]
fn host_list() -> Vec<String> {
	debug!("Host list command invoked");

	// get cpal hosts list
	let hosts = cpal::available_hosts();
	let mut output_hosts: Vec<String> = Vec::new();
	debug!("Available hosts:");
	for host_id in hosts {
		debug!("{:?}", host_id);
		output_hosts.push(host_id.name().to_string());
	}
	output_hosts
}

#[tauri::command]
fn device_list(hostname: String) -> Vec<String> {
	debug!("Device list command invoked");

	// get cpal hosts list
	let hosts = cpal::available_hosts();
	let host_id = hosts
		.iter()
		.find(|h| h.name().to_string() == hostname)
		.unwrap();

	let host = cpal::host_from_id(*host_id).unwrap();

	let devices = host.devices().unwrap();
	let mut output_devices: Vec<String> = Vec::new();
	debug!("Available devices:");
	for device in devices {
		debug!("{:?}", device.name().unwrap());
		output_devices.push(device.name().unwrap());
	}
	output_devices
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
			host_list,
			device_list,
			exit,
		])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
