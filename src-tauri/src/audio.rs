//! audio.rs
//! 
//! Module is used for interacting with audio drivers/hardware
//! 
//! ## Functions
//! 
//! * `get_host(host_name: &str) -> Host` - Gets either the desired hostname, or if it is unavailable, the default host. Will also return the default host if "default" is entered. Host name is not case-sensitive.
//! * `get_output_device(device_name: &str, host: &Host) -> Option<Device>` - Gets either the desired output device, or if it is unavailable, the default output device. Will also return the default output device if "default" is entered. Device name is not case-sensitive.
//! * `get_input_device(device_name: &str, host: &Host) -> Option<Device>` - Gets either the desired input device, or if it is unavailable, the default input device. Will also return the default input device if "default" is entered. Device name is not case-sensitive.

use cpal::{Host, Device, traits::{HostTrait, DeviceTrait}};
use log::debug;

/// ## get_host(host_name: &str) -> Host
/// 
/// Gets either the desired hostname, or if it is unavailable, the default host.
/// Will also return the default host if "default" is entered.
/// Host name is not case-sensitive.
/// 
/// ### Arguments
/// 
/// * `host_name: &str` - The name of the desired host
/// 
/// ### Returns
/// 
/// * `Host` - The resulting host
/// 
/// ### Examples
/// 
/// ```
/// let host = audio::get_host("CoreAudio");
/// ```
fn get_host(host_name: &str) -> Host {
	debug!("Getting host {}", host_name);
	let default_host = cpal::default_host();

	if host_name.to_lowercase() == "default" {
		debug!("Returning default host {}.", default_host.id().name());
		return default_host;
	}

	let host_ids = cpal::available_hosts();
	for host_id in host_ids {
		let host_id_name = host_id.name();
		if host_id_name.to_lowercase() == host_name.to_lowercase() {
			let host = cpal::host_from_id(host_id);
			match host {
				Ok(host) => {
					debug!("Returned host {}", host_id_name);
					return host;
				}
				Err(err) => {
					debug!("Failed to get host due to error {}\nSearching for more hosts...", err.to_string());
				}
			}
		}
	}

	debug!("Could not find host {}. Returning default host {}.", host_name, default_host.id().name());
	cpal::default_host()
}

/// ## get_output_device(device_name: &str, host: &Host) -> Option<Device>
/// 
/// Gets either the desired output device, or if it is unavailable, the default output device.
/// Will also return the default output device if "default" is entered.
/// Device name is not case-sensitive.
/// 
/// ### Arguments
/// 
/// * `device_name: &str` - The name of the desired output device
/// * `host: &Host` - The host to search for the device on
/// 
/// ### Returns
/// 
/// * `Option<Device>` - The resulting device
/// 
/// ### Examples
/// 
/// ```
/// let host = audio::get_host("CoreAudio");
/// let device = audio::get_output_device("Macbook Air Speakers", &host);
/// ```
fn get_output_device(device_name: &str, host: &Host) -> Option<Device> {
	let default_device = host.default_output_device();
	let default_device_name = match &default_device {
		Some(device) => {
			match device.name() {
				Ok(name) => {
					name
				}
				Err(err) => {
					"Unknown".to_owned()
				}
			}
		}
		None => {
			"None".to_owned()
		}
	};

	if device_name.to_lowercase() == "default" {
		debug!("Returning default output device {}", default_device_name);
		return default_device;
	}

	let devices = host.output_devices();

	let devices = match devices {
		Ok(devices) => {
			devices
		}
		Err(err) => {
			debug!("Error getting output devices. returning default output device {}.", default_device_name);
			return default_device;
		}
	};

	for device in devices {
		match device.name() {
			Ok(name) => {
				if name.to_lowercase() == device_name.to_lowercase() {
					debug!("Returning output device {}", name);
					return Some(device);
				}
			}
			Err(err) => {
				debug!("Error retrieving output device name.");
			}
		}
	}

	debug!("Could not find output device {}. Returning default output device {}.", device_name, default_device_name);
	default_device
}

/// ## get_input_device(device_name: &str, host: &Host) -> Option<Device>
/// 
/// Gets either the desired input device, or if it is unavailable, the default input device.
/// Will also return the default input device if "default" is entered.
/// Device name is not case-sensitive.
/// 
/// ### Arguments
/// 
/// * `device_name: &str` - The name of the desired input device
/// * `host: &Host` - The host to search for the device on
/// 
/// ### Returns
/// 
/// * `Option<Device>` - The resulting device
/// 
/// ### Examples
/// 
/// ```
/// let host = audio::get_host("CoreAudio");
/// let device = audio::get_input_device("Macbook Air Microphone", &host);
/// ```
fn get_input_device(device_name: &str, host: &Host) -> Option<Device> {
	let default_device = host.default_input_device();
	let default_device_name = match &default_device {
		Some(device) => {
			match device.name() {
				Ok(name) => {
					name
				}
				Err(err) => {
					"Unknown".to_owned()
				}
			}
		}
		None => {
			"None".to_owned()
		}
	};

	if device_name.to_lowercase() == "default" {
		debug!("Returning default input device {}", default_device_name);
		return default_device;
	}

	let devices = host.input_devices();

	let devices = match devices {
		Ok(devices) => {
			devices
		}
		Err(err) => {
			debug!("Error getting input devices. returning default input device {}.", default_device_name);
			return default_device;
		}
	};

	for device in devices {
		match device.name() {
			Ok(name) => {
				if name.to_lowercase() == device_name.to_lowercase() {
					debug!("Returning input device {}", name);
					return Some(device);
				}
			}
			Err(err) => {
				debug!("Error retrieving input device name.");
			}
		}
	}

	debug!("Could not find input device {}. Returning default input device {}.", device_name, default_device_name);
	default_device
}