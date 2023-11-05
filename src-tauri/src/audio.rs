//! audio.rs
//!
//! Module is used for interacting with audio drivers/hardware
//!
//! ## Functions
//!
//! * `get_host(host_name: &str) -> Host` - Gets either the desired hostname, or if it is unavailable, the default host. Will also return the default host if "default" is entered. Host name is not case-sensitive.
//! * `get_output_device(device_name: &str, host: &Host) -> Option<Device>` - Gets either the desired output device, or if it is unavailable, the default output device. Will also return the default output device if "default" is entered. Device name is not case-sensitive.
//! * `get_input_device(device_name: &str, host: &Host) -> Option<Device>` - Gets either the desired input device, or if it is unavailable, the default input device. Will also return the default input device if "default" is entered. Device name is not case-sensitive.

use std::sync::Mutex;

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Host,
};
use lazy_static::lazy_static;
use log::debug;

lazy_static! {
    pub static ref HOST: Mutex<Option<cpal::Host>> = Mutex::new(None);
    pub static ref OUTPUT: Mutex<Option<cpal::Device>> = Mutex::new(None);
    pub static ref INPUT: Mutex<Option<cpal::Device>> = Mutex::new(None);
}

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
pub fn get_host(host_name: &str) -> Host {
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
                    debug!(
                        "Failed to get host due to error {}\nSearching for more hosts...",
                        err.to_string()
                    );
                }
            }
        }
    }

    debug!(
        "Could not find host {}. Returning default host {}.",
        host_name,
        default_host.id().name()
    );
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
pub fn get_output_device(device_name: &str, host: &Host) -> Option<Device> {
    let default_device = host.default_output_device();
    let default_device_name = match &default_device {
        Some(device) => match device.name() {
            Ok(name) => name,
            Err(err) => "Unknown".to_owned(),
        },
        None => "None".to_owned(),
    };

    if device_name.to_lowercase() == "default" {
        debug!("Returning default output device {}", default_device_name);
        return default_device;
    }

    let devices = host.output_devices();

    let devices = match devices {
        Ok(devices) => devices,
        Err(err) => {
            debug!(
                "Error getting output devices. returning default output device {}.",
                default_device_name
            );
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

    debug!(
        "Could not find output device {}. Returning default output device {}.",
        device_name, default_device_name
    );
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
pub fn get_input_device(device_name: &str, host: &Host) -> Option<Device> {
    let default_device = host.default_input_device();
    let default_device_name = match &default_device {
        Some(device) => match device.name() {
            Ok(name) => name,
            Err(err) => "Unknown".to_owned(),
        },
        None => "None".to_owned(),
    };

    if device_name.to_lowercase() == "default" {
        debug!("Returning default input device {}", default_device_name);
        return default_device;
    }

    let devices = host.input_devices();

    let devices = match devices {
        Ok(devices) => devices,
        Err(err) => {
            debug!(
                "Error getting input devices. returning default input device {}.",
                default_device_name
            );
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

    debug!(
        "Could not find input device {}. Returning default input device {}.",
        device_name, default_device_name
    );
    default_device
}

/// ## list_hosts() -> Vec<String>
///
/// Lists all available hosts.
///
/// ### Returns
///
/// * `Vec<String>` - The list of hosts
pub fn list_hosts() -> Vec<String> {
    let mut hosts = Vec::new();
    let host_ids = cpal::available_hosts();
    for host_id in host_ids {
        let host_id_name = host_id.name();
        hosts.push(host_id_name.to_owned());
    }
    hosts
}

/// ## list_output_devices(host: &Host) -> Vec<String>
///
/// Lists all available output devices on a host.
///
/// ### Arguments
///
/// * `host: &Host` - The host to search for devices on
///
/// ### Returns
///
/// * `Vec<String>` - The list of output devices
pub fn list_output_devices(host: &Host) -> Vec<String> {
    let mut devices = Vec::new();
    let output_devices = host.output_devices();
    let output_devices = match output_devices {
        Ok(output_devices) => output_devices,
        Err(err) => {
            debug!("Error getting output devices: {}", err);
            return devices;
        }
    };
    for output_device in output_devices {
        let output_device_name = output_device.name();
        let output_device_name = match output_device_name {
            Ok(output_device_name) => output_device_name,
            Err(err) => {
                debug!("Error getting output device name: {}", err);
                continue;
            }
        };
        devices.push(output_device_name);
    }
    devices
}

/// ## list_input_devices(host: &Host) -> Vec<String>
///
/// Lists all available input devices on a host.
///
/// ### Arguments
///
/// * `host: &Host` - The host to search for devices on
///
/// ### Returns
///
/// * `Vec<String>` - The list of input devices
pub fn list_input_devices(host: &Host) -> Vec<String> {
    let mut devices = Vec::new();
    let input_devices = host.input_devices();
    let input_devices = match input_devices {
        Ok(input_devices) => input_devices,
        Err(err) => {
            debug!("Error getting input devices: {}", err);
            return devices;
        }
    };
    for input_device in input_devices {
        let input_device_name = input_device.name();
        let input_device_name = match input_device_name {
            Ok(input_device_name) => input_device_name,
            Err(err) => {
                debug!("Error getting input device name: {}", err);
                continue;
            }
        };
        devices.push(input_device_name);
    }
    devices
}

struct Channel {
    direction: Direction,
    channel_id: u16,
}

enum Direction {
    Input,
    Output,
    None,
}

enum Channels {
    Mono(Channel),
    Stereo(Channel, Channel),
    Custom(u16),
}
