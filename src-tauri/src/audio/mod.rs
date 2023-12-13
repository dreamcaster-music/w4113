//! audio.rs
//!
//! Module is used for interacting with audio drivers/hardware

#![allow(dead_code)]

use std::sync::{Arc, Mutex, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, Device, Host, SupportedStreamConfigRange,
};
use lazy_static::lazy_static;
use log::debug;
use tauri::Manager;
use ts_rs::TS;

use crate::tv::{BasicVisualizer, VisualizerTrait};

use self::plugin::{SampleGenerator, Command};

pub mod plugin;

lazy_static! {
    pub static ref HOST: Mutex<Option<cpal::Host>> = Mutex::new(None);
    pub static ref OUTPUT_DEVICE: Mutex<Option<cpal::Device>> = Mutex::new(None);
    pub static ref INPUT_DEVICE: Mutex<Option<cpal::Device>> = Mutex::new(None);
    pub static ref OUTPUT_CONFIG: Mutex<Option<cpal::StreamConfig>> = Mutex::new(None);
    pub static ref INPUT_CONFIG: Mutex<Option<cpal::StreamConfig>> = Mutex::new(None);
    pub static ref STRIPS: RwLock<Vec<Strip>> = RwLock::new(Vec::new());
    pub static ref RELOAD: RwLock<bool> = RwLock::new(false);
    pub static ref AUDIO_THREAD: Mutex<Option<std::thread::JoinHandle<Result<(), String>>>> =
        Mutex::new(None);
}

fn force_reload() {
    let mut reload = match RELOAD.write() {
        Ok(reload) => reload,
        Err(e) => {
            debug!("Error locking RELOAD: {}", e);
            return;
        }
    };

    *reload = true;
}

/// ## `get_host(host_name: &str) -> Host`
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
    let default_host = cpal::default_host();

    if host_name.to_lowercase() == "default" {
        return default_host;
    }

    let host_ids = cpal::available_hosts();
    for host_id in host_ids {
        let host_id_name = host_id.name();
        if host_id_name.to_lowercase() == host_name.to_lowercase() {
            let host = cpal::host_from_id(host_id);
            match host {
                Ok(host) => {
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
        "Could not find host '{}'. Returning default host '{}'.",
        host_name,
        default_host.id().name()
    );
    cpal::default_host()
}

/// ## `get_output_device(device_name: &str, host: &Host) -> Option<Device>`
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
            Err(_err) => "Unknown".to_owned(),
        },
        None => "None".to_owned(),
    };

    if device_name.to_lowercase() == "default" {
        return default_device;
    }

    let devices = host.output_devices();

    let devices = match devices {
        Ok(devices) => devices,
        Err(_err) => {
            debug!(
                "Error getting output devices. returning default output device '{}'.",
                default_device_name
            );
            return default_device;
        }
    };

    for device in devices {
        match device.name() {
            Ok(name) => {
                if name.to_lowercase() == device_name.to_lowercase() {
                    return Some(device);
                }
            }
            Err(_err) => {
                debug!("Error retrieving output device name.");
            }
        }
    }

    debug!(
        "Could not find output device '{}'. Returning default output device '{}'.",
        device_name, default_device_name
    );
    default_device
}

/// ## `get_input_device(device_name: &str, host: &Host) -> Option<Device>`
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
            Err(_err) => "Unknown".to_owned(),
        },
        None => "None".to_owned(),
    };

    if device_name.to_lowercase() == "default" {
        return default_device;
    }

    let devices = host.input_devices();

    let devices = match devices {
        Ok(devices) => devices,
        Err(_err) => {
            debug!(
                "Error getting input devices. returning default input device '{}'.",
                default_device_name
            );
            return default_device;
        }
    };

    for device in devices {
        match device.name() {
            Ok(name) => {
                if name.to_lowercase() == device_name.to_lowercase() {
                    return Some(device);
                }
            }
            Err(_err) => {
                debug!("Error retrieving input device name.");
            }
        }
    }

    debug!(
        "Could not find input device '{}'. Returning default input device '{}'.",
        device_name, default_device_name
    );
    default_device
}

/// ## `list_hosts() -> Vec<String>`
///
/// Lists all available hosts.
///
/// ### Returns
///
/// * `Vec<String>` - The list of hosts
#[tauri::command]
pub fn list_hosts() -> Vec<String> {
    let mut hosts = Vec::new();
    let host_ids = cpal::available_hosts();
    for host_id in host_ids {
        let host_id_name = host_id.name();
        hosts.push(host_id_name.to_owned());
    }
    hosts
}

/// Sets the host.
///
/// ### Arguments
///
/// * `name: String` - The name of the host to set
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
#[tauri::command]
pub fn set_host(name: String) -> Result<(), String> {
    let host = get_host(&name);
    let mut mutex = match HOST.lock() {
        Ok(host) => host,
        Err(e) => {
            debug!("Error locking HOST: {}", e);
            return Err(format!("Error locking HOST: {}", e));
        }
    };
	let name = host.id().name().to_string();

    *mutex = Some(host);

    let mut config = match crate::CONFIG.write() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return Err(format!("Error locking CONFIG: {}", e));
        }
    };

    match config.set("audio.host", name.as_str()) {
        Ok(_) => {}
        Err(e) => {
            debug!("Error setting audio.host: {}", e);
            return Err(format!("Error setting audio.host: {}", e));
        }
    }

	debug!("Set host to {}", name);
    force_reload();

    Ok(())
}

/// Returns the name of the host.
///
/// ### Returns
///
/// * `String` - The name of the host
pub fn host() -> String {
    let host = match HOST.lock() {
        Ok(host) => host,
        Err(e) => {
            debug!("Error locking HOST: {}", e);
            return "Error".to_string();
        }
    };

    let host = match host.as_ref() {
        Some(host) => host,
        None => {
            debug!("HOST is None");
            return "None".to_string();
        }
    };

    let host_name = host.id().name().to_owned();

    host_name
}

/// ## `list_output_devices(host: &Host) -> Vec<String>`
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
#[tauri::command]
pub fn list_output_devices() -> Vec<String> {
    let host = match HOST.lock() {
        Ok(host) => host,
        Err(e) => {
            debug!("Error locking HOST: {}", e);
            return Vec::new();
        }
    };

    let host = match host.as_ref() {
        Some(host) => host,
        None => {
            debug!("HOST is None");
            return Vec::new();
        }
    };

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

/// Sets the output device.
///
/// ### Arguments
///
/// * `name: String` - The name of the output device to set
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
#[tauri::command]
pub fn set_output_device(name: String) -> Result<(), String> {
    let host = match HOST.lock() {
        Ok(host) => host,
        Err(e) => {
            debug!("Error locking HOST: {}", e);
            return Err(format!("Error locking HOST: {}", e));
        }
    };

    let host = match host.as_ref() {
        Some(host) => host,
        None => {
            debug!("HOST is None");
            return Err("HOST is None".to_owned());
        }
    };

    let device = get_output_device(&name, &host);
    let device = match device {
        Some(device) => device,
        None => {
            debug!("Could not find output device {}", name);
            return Err(format!("Could not find output device {}", name));
        }
    };

	let name = match device.name() {
		Ok(name) => name,
		Err(e) => {
			debug!("Error getting input device name: {}", e);
			"Error".to_string()
		}
	};

    let mut mutex = match OUTPUT_DEVICE.lock() {
        Ok(output_device) => output_device,
        Err(e) => {
            debug!("Error locking OUTPUT_DEVICE: {}", e);
            return Err(format!("Error locking OUTPUT_DEVICE: {}", e));
        }
    };

    *mutex = Some(device);

    let mut config = match crate::CONFIG.write() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return Err(format!("Error locking CONFIG: {}", e));
        }
    };

    match config.set("audio.output.device", name.as_str()) {
        Ok(_) => {}
        Err(e) => {
            debug!("Error setting audio.output_device: {}", e);
            return Err(format!("Error setting audio.output_device: {}", e));
        }
    }

	debug!("Set output device to {}", name);
    force_reload();

    Ok(())
}

/// Returns the name of the output device.
///
/// ### Returns
///
/// * `String` - The name of the output device
pub fn output_device() -> String {
    let device = match OUTPUT_DEVICE.lock() {
        Ok(device) => device,
        Err(e) => {
            debug!("Error locking OUTPUT_DEVICE: {}", e);
            return "Error".to_string();
        }
    };

    let device = match device.as_ref() {
        Some(device) => device,
        None => {
            debug!("OUTPUT_DEVICE is None");
            return "None".to_string();
        }
    };

    let device_name = device.name().unwrap();

    device_name
}

/// ## `list_input_devices(host: &Host) -> Vec<String>`
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
#[tauri::command]
pub fn list_input_devices() -> Vec<String> {
    let host = match HOST.lock() {
        Ok(host) => host,
        Err(e) => {
            debug!("Error locking HOST: {}", e);
            return Vec::new();
        }
    };

    let host = match host.as_ref() {
        Some(host) => host,
        None => {
            debug!("HOST is None");
            return Vec::new();
        }
    };

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

/// Sets the input device.
///
/// ### Arguments
///
/// * `name: String` - The name of the input device to set
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
#[tauri::command]
pub fn set_input_device(name: String) -> Result<(), String> {
    let host = match HOST.lock() {
        Ok(host) => host,
        Err(e) => {
            debug!("Error locking HOST: {}", e);
            return Err(format!("Error locking HOST: {}", e));
        }
    };

    let host = match host.as_ref() {
        Some(host) => host,
        None => {
            debug!("HOST is None");
            return Err("HOST is None".to_owned());
        }
    };

    let device = get_input_device(&name, &host);
    let device = match device {
        Some(device) => device,
        None => {
            debug!("Could not find input device {}", name);
            return Err(format!("Could not find input device {}", name));
        }
    };

	let name = match device.name() {
		Ok(name) => name,
		Err(e) => {
			debug!("Error getting input device name: {}", e);
			"Error".to_string()
		}
	};

    let mut mutex = match INPUT_DEVICE.lock() {
        Ok(input_device) => input_device,
        Err(e) => {
            debug!("Error locking INPUT_DEVICE: {}", e);
            return Err(format!("Error locking INPUT_DEVICE: {}", e));
        }
    };

    *mutex = Some(device);

    let mut config = match crate::CONFIG.write() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return Err(format!("Error locking CONFIG: {}", e));
        }
    };

    match config.set("audio.input.device", name.as_str()) {
        Ok(_) => {}
        Err(e) => {
            debug!("Error setting audio.input.device: {}", e);
            return Err(format!("Error setting audio.input.device: {}", e));
        }
    }

	debug!("Set input device to {}", name);
    force_reload();

    Ok(())
}

/// Returns the name of the input device.
///
/// ### Returns
///
/// * `String` - The name of the input device
pub fn input_device() -> String {
    let device = match INPUT_DEVICE.lock() {
        Ok(device) => device,
        Err(e) => {
            debug!("Error locking INPUT_DEVICE: {}", e);
            return "Error".to_string();
        }
    };

    let device = match device.as_ref() {
        Some(device) => device,
        None => {
            debug!("INPUT_DEVICE is None");
            return "None".to_string();
        }
    };

    let device_name = device.name().unwrap();

    device_name
}

/*
pub fn play_sound_file(path: String) -> Result<(), String> {
    Ok(())
}
*/

/// ## PreferenceAlt
///
/// If the higher priority "Preference" is unavailable, this enum is used to
/// indicate whether the closest higher or lower value should be used instead.
///
/// ### Variants
///
/// * `Higher` - The preference should be higher than the given value
/// * `Lower` - The preference should be lower than the given value
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum PreferenceAlt {
    Higher,
    Lower,
}

/// ## Preference
///
/// Used to indicate a preference for a given value.
///
/// ### Variants
///
/// * `Min` - The minimum value should be used
/// * `Max` - The maximum value should be used
/// * `Exact(u32, PreferenceAlt)` - The exact value should be used, or if it is unavailable, the closest higher or lower value should be used instead
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Preference {
    Min,
    Max,
    Exact(u32, PreferenceAlt),
}

/// ## ConfigProperty
///
/// Used to indicate which property of a config should be filtered.
///
/// ### Variants
///
/// * `Channels(Preference)` - The number of channels
/// * `SampleRate(Preference)` - The sample rate
/// * `BufferSize(Preference)` - The buffer size
#[derive(Clone, Debug)]
enum ConfigProperty {
    Channels(Preference),
    SampleRate(Preference),
    BufferSize(Preference),
}

/// ## `filter_config(configs_ref: Vec<SupportedStreamConfigRange>, property: ConfigProperty, alt: bool) -> Vec<SupportedStreamConfigRange>`
///
/// Filters a list of configs based on the given property.
///
/// ### Arguments
///
/// * `configs_ref: Vec<SupportedStreamConfigRange>` - The list of configs to filter
/// * `property: ConfigProperty` - The property to filter by
/// * `alt: bool` - Whether or not to use the alternate preference if the exact preference is unavailable. Should always be false when calling this function.
///
/// ### Returns
///
/// * `Vec<SupportedStreamConfigRange>` - The resulting list of configs
///
/// ### Examples
///
/// ```
/// let mut configs = filter_config(configs, ConfigProperty::Channels(Preference::Exact(2, PreferenceAlt::Higher)), false);
/// configs = filter_config(configs, ConfigProperty::SampleRate(Preference::Exact(44100, PreferenceAlt::Higher)), false);
/// configs = filter_config(configs, ConfigProperty::BufferSize(Preference::Exact(1024, PreferenceAlt::Higher)), false);
/// ```
fn filter_config(
    configs_ref: Vec<SupportedStreamConfigRange>,
    property: ConfigProperty,
    alt: bool,
) -> Vec<SupportedStreamConfigRange> {
    let mut configs: Vec<SupportedStreamConfigRange> = Vec::new();

    let preference = match property.clone() {
        ConfigProperty::Channels(channels) => channels,
        ConfigProperty::SampleRate(sample_rate) => sample_rate,
        ConfigProperty::BufferSize(buffer_size) => buffer_size,
    };

    let mut comparison_value;
    let mut exact_value = 0;
    match preference {
        Preference::Max => {
            comparison_value = std::u32::MIN;
        }
        Preference::Min => {
            comparison_value = std::u32::MAX;
        }
        Preference::Exact(value, ref _preference_alt) => {
            exact_value = value;
            if alt {
                match _preference_alt {
                    PreferenceAlt::Higher => {
                        comparison_value = std::u32::MAX;
                    }
                    PreferenceAlt::Lower => {
                        comparison_value = std::u32::MIN;
                    }
                }
            } else {
                comparison_value = value;
            }
        }
    }

    for config in configs_ref.clone() {
        let max_config_value;
        let min_config_value;
        match &property {
            ConfigProperty::Channels(_channels) => {
                let config_channels = config.channels();
                max_config_value = config_channels as u32;
                min_config_value = config_channels as u32;
            }
            ConfigProperty::SampleRate(_sample_rate) => {
                max_config_value = config.max_sample_rate().0;
                min_config_value = config.min_sample_rate().0;
            }
            ConfigProperty::BufferSize(_buffer_size) => {
                let config_buffer_size = config.buffer_size();
                let config_buffer_size = match config_buffer_size {
                    cpal::SupportedBufferSize::Range { min, max } => (*min, *max),
                    cpal::SupportedBufferSize::Unknown => (0, 0),
                };
                max_config_value = config_buffer_size.1;
                min_config_value = config_buffer_size.0;
            }
        }

        match preference {
            Preference::Max => {
                if max_config_value > comparison_value {
                    comparison_value = max_config_value;
                    configs.clear();
                    configs.push(config);
                } else if max_config_value == comparison_value {
                    configs.push(config);
                }
            }
            Preference::Min => {
                if min_config_value < comparison_value {
                    comparison_value = min_config_value;
                    configs.clear();
                    configs.push(config);
                } else if min_config_value == comparison_value {
                    configs.push(config);
                }
            }
            Preference::Exact(value, ref preference_alt) => {
                if alt {
                    match preference_alt {
                        PreferenceAlt::Higher => {
                            if value < comparison_value && value > exact_value {
                                comparison_value = value;
                                configs.clear();
                                configs.push(config);
                            } else if value == comparison_value {
                                configs.push(config);
                            }
                        }
                        PreferenceAlt::Lower => {
                            if value > comparison_value && value < exact_value {
                                comparison_value = value;
                                configs.clear();
                                configs.push(config);
                            } else if value == comparison_value {
                                configs.push(config);
                            }
                        }
                    }
                } else {
                    if value <= max_config_value && value >= min_config_value {
                        configs.push(config);
                    }
                }
            }
        }
    }

    if configs.len() < 1 {
        if alt {
            // TODO: Handle this better
        } else {
            return filter_config(configs_ref, property, true);
        }
    }

    configs
}

/// ## `get_output_config(device: Device, channels: Preference, sample_rate: Preference, buffer_size: Preference) -> Option<cpal::StreamConfig>`
///
/// Gets the output config for the given device, channels, and sample rate.
/// Notably, "channels" takes precedence over "sample_rate", which takes precedence over "buffer_size".
///
/// ### Arguments
///
/// * `device: Device` - The device to get the config for
/// * `channels: Preference` - The desired number of channels
/// * `sample_rate: Preference` - The desired sample rate
/// * `buffer_size: Preference` - The desired buffer size
///
/// ### Returns
///
/// * `Option<cpal::StreamConfig>` - The resulting config
///
/// ### Examples
///
/// ```
/// let host = audio::get_host("CoreAudio");
/// let device = audio::get_output_device("Macbook Air Speakers", &host);
/// let config = audio::get_output_config(device, Preference::Exact(2, PreferenceAlt::Higher), Preference::Exact(44100, PreferenceAlt::Higher), Preference::Exact(1024, PreferenceAlt::Higher));
/// ```
pub fn get_output_config(
    device: &Device,
    channels: Preference,
    sample_rate: Preference,
    buffer_size: Preference,
) -> Option<cpal::StreamConfig> {
    let default = device.default_output_config();

    let supported_configs = match device.supported_output_configs() {
        Ok(supported_configs) => supported_configs,
        Err(err) => {
            debug!("Error getting supported output configs: {}", err);
            return Some(default.ok()?.config());
        }
    };

    let mut supported_configs = supported_configs.collect::<Vec<_>>();

    /* debug!("Enumerating configs for device {}...", device.name().unwrap());
    for config in supported_configs.clone() {
        debug!("Config properties:\n\tChannels: {}\n\tMin Sample Rate: {}\n\tMax Sample Rate: {}\n\tBuffer Size: {:?}", config.channels(), config.min_sample_rate().0, config.max_sample_rate().0, config.buffer_size());
    } */

    supported_configs = filter_config(supported_configs, ConfigProperty::Channels(channels), false);
    supported_configs = filter_config(
        supported_configs,
        ConfigProperty::SampleRate(sample_rate.clone()),
        false,
    );
    supported_configs = filter_config(
        supported_configs,
        ConfigProperty::BufferSize(buffer_size.clone()),
        false,
    );

    let first = supported_configs.first();
    let first = match first {
        Some(first) => first.clone(),
        None => {
            debug!("No supported output configs found.");
            return Some(default.ok()?.config());
        }
    };

    let max = first.max_sample_rate().0;
    let min = first.min_sample_rate().0;

    let config = match sample_rate {
        Preference::Exact(value, _preference_alt) => {
            if value > max {
                first.with_sample_rate(cpal::SampleRate(max))
            } else if value < min {
                first.with_sample_rate(cpal::SampleRate(min))
            } else {
                first.with_sample_rate(cpal::SampleRate(value))
            }
        }
        Preference::Max => first.with_max_sample_rate(),
        Preference::Min => {
            let min = &first.min_sample_rate();
            first.with_sample_rate(*min)
        }
    };
    let mut config = config.config();
    config.buffer_size = match buffer_size {
        Preference::Exact(value, _preference_alt) => BufferSize::Fixed(value as u32),
        Preference::Max => BufferSize::Default,
        Preference::Min => BufferSize::Default,
    };
    Some(config)
}

/// ## `get_input_config(device: Device, channels: Preference, sample_rate: Preference, buffer_size: Preference) -> Option<cpal::StreamConfig>`
///
/// Gets the input config for the given device, channels, and sample rate.
///
/// ### Arguments
///
/// * `device: Device` - The device to get the config for
/// * `channels: Preference` - The desired number of channels
/// * `sample_rate: Preference` - The desired sample rate
/// * `buffer_size: Preference` - The desired buffer size
///
/// ### Returns
///
/// * `Option<cpal::StreamConfig>` - The resulting config
///
/// ### Examples
///
/// ```
/// let host = audio::get_host("CoreAudio");
/// let device = audio::get_input_device("Macbook Air Microphone", &host);
/// let config = audio::get_input_config(device, Preference::Exact(2, PreferenceAlt::Higher), Preference::Exact(44100, PreferenceAlt::Higher), Preference::Exact(1024, PreferenceAlt::Higher));
/// ```
pub fn get_input_config(
    device: &Device,
    channels: Preference,
    sample_rate: Preference,
    buffer_size: Preference,
) -> Option<cpal::StreamConfig> {
    let default = device.default_input_config();
    let supported_configs = match device.supported_input_configs() {
        Ok(supported_configs) => supported_configs,
        Err(err) => {
            debug!("Error getting supported input configs: {}", err);
            return Some(default.ok()?.config());
        }
    };

    let mut supported_configs = supported_configs.collect::<Vec<_>>();

    /* debug!("Enumerating configs for device {}...", device.name().unwrap());
    for config in supported_configs.clone() {
        debug!("Config properties:\n\tChannels: {}\n\tMin Sample Rate: {}\n\tMax Sample Rate: {}\n\tBuffer Size: {:?}", config.channels(), config.min_sample_rate().0, config.max_sample_rate().0, config.buffer_size());
    } */

    supported_configs = filter_config(supported_configs, ConfigProperty::Channels(channels), false);
    supported_configs = filter_config(
        supported_configs,
        ConfigProperty::SampleRate(sample_rate.clone()),
        false,
    );
    supported_configs = filter_config(
        supported_configs,
        ConfigProperty::BufferSize(buffer_size.clone()),
        false,
    );

    let first = supported_configs.first();
    let first = match first {
        Some(first) => first.clone(),
        None => {
            debug!("No supported input configs found.");
            return Some(default.ok()?.config());
        }
    };

    let max = first.max_sample_rate().0;
    let min = first.min_sample_rate().0;

    let config = match sample_rate {
        Preference::Exact(value, _preference_alt) => {
            if value > max {
                first.with_sample_rate(cpal::SampleRate(max))
            } else if value < min {
                first.with_sample_rate(cpal::SampleRate(min))
            } else {
                first.with_sample_rate(cpal::SampleRate(value))
            }
        }
        Preference::Max => first.with_max_sample_rate(),
        Preference::Min => {
            let min = &first.min_sample_rate();
            first.with_sample_rate(*min)
        }
    };
    let mut config = config.config();
    config.buffer_size = match buffer_size {
        Preference::Exact(value, _preference_alt) => BufferSize::Fixed(value as u32),
        Preference::Max => BufferSize::Default,
        Preference::Min => BufferSize::Default,
    };
    Some(config)
}

/// ## `list_output_streams(device: &Device) -> Result<Vec<String>, String>`
///
/// Lists all available output stream configurations for a device.
///
/// ### Arguments
///
/// * `device: &Device` - The device to list the output stream configurations for
///
/// ### Returns
///
/// * `Result<Vec<String>, String>` - The list of output stream configurations, or an error message
#[tauri::command]
pub fn list_output_streams() -> Vec<String> {
    let device = match OUTPUT_DEVICE.lock() {
        Ok(device) => device,
        Err(e) => {
            debug!("Error locking OUTPUT_DEVICE: {}", e);
            return Vec::new();
        }
    };

    let device = match device.as_ref() {
        Some(device) => device,
        None => {
            debug!("OUTPUT_DEVICE is None");
            return Vec::new();
        }
    };

    let supported_configs = match device.supported_output_configs() {
        Ok(supported_configs) => supported_configs,
        Err(err) => return vec![format!("Error getting supported output configs.")],
    };

    let mut streams = Vec::new();
    for config in supported_configs {
        let channels = config.channels();
        let sample_rate = config.min_sample_rate().0;
        let buffer_size = config.buffer_size();
        let buffer_size = match buffer_size {
            cpal::SupportedBufferSize::Range { min, max } => (*min, *max),
            cpal::SupportedBufferSize::Unknown => (0, 0),
        };
        let stream = format!(
            "{} {} {}-{}",
            channels, sample_rate, buffer_size.0, buffer_size.1
        );
        streams.push(stream);
    }

    streams
}

/// ## `list_input_streams(device: &Device) -> Result<Vec<String>, String>`
///
/// Lists all available input stream configurations for a device.
///
/// ### Arguments
///
/// * `device: &Device` - The device to list the input stream configurations for
///
/// ### Returns
///
/// * `Result<Vec<String>, String>` - The list of input stream configurations, or an error message
#[tauri::command]
pub fn list_input_streams() -> Vec<String> {
    let device = match INPUT_DEVICE.lock() {
        Ok(device) => device,
        Err(e) => {
            debug!("Error locking INPUT_DEVICE: {}", e);
            return Vec::new();
        }
    };

    let device = match device.as_ref() {
        Some(device) => device,
        None => {
            debug!("INPUT_DEVICE is None");
            return Vec::new();
        }
    };

    let supported_configs = match device.supported_input_configs() {
        Ok(supported_configs) => supported_configs,
        Err(err) => return vec![format!("Error getting supported input configs.")],
    };

    let mut streams = Vec::new();
    for config in supported_configs {
        let channels = config.channels();
        let sample_rate = config.min_sample_rate().0;
        let buffer_size = config.buffer_size();
        let buffer_size = match buffer_size {
            cpal::SupportedBufferSize::Range { min, max } => (*min, *max),
            cpal::SupportedBufferSize::Unknown => (0, 0),
        };
        let stream = format!(
            "{} {} {}-{}",
            channels, sample_rate, buffer_size.0, buffer_size.1
        );
        streams.push(stream);
    }

    streams
}

/// Sets the output stream.
///
/// ### Arguments
///
/// * `stream: String` - The stream to set, in the format "channels sample_rate buffer_size_min-buffer_size_max"
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
#[tauri::command]
pub fn set_output_stream(stream: String) -> Result<(), String> {
    let split = stream
        .split(' ')
        .filter(|e| e.len() > 0)
        .collect::<Vec<_>>();
    if split.len() < 3 {
        return Err(format!("Invalid stream format: {}", stream));
    }
    let channels = split[0].parse::<u32>().unwrap();
    let sample_rate = split[1].parse::<u32>().unwrap();
    let buffer_size = split[2].split('-').collect::<Vec<_>>();
    let buffer_size_min = buffer_size[0].parse::<u32>().unwrap();
    let buffer_size_max = buffer_size[1].parse::<u32>().unwrap();

    let device = match OUTPUT_DEVICE.lock() {
        Ok(device) => device,
        Err(e) => {
            debug!("Error locking OUTPUT_DEVICE: {}", e);
            return Err(format!("Error locking OUTPUT_DEVICE: {}", e));
        }
    };

    let stream_config = get_output_config(
        device.as_ref().unwrap(),
        Preference::Exact(channels, PreferenceAlt::Higher),
        Preference::Exact(sample_rate, PreferenceAlt::Higher),
        Preference::Exact(buffer_size_max, PreferenceAlt::Higher),
    );
    match stream_config {
        Some(stream_config) => {
            let mut config = match OUTPUT_CONFIG.lock() {
                Ok(config) => config,
                Err(e) => {
                    debug!("Error locking OUTPUT_CONFIG: {}", e);
                    return Err(format!("Error locking OUTPUT_CONFIG: {}", e));
                }
            };
            *config = Some(stream_config);
        }
        None => {
            return Err(format!("Could not find output stream {}", stream));
        }
    }

    let mut config = match crate::CONFIG.write() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return Err(format!("Error locking CONFIG: {}", e));
        }
    };

    config.set(
        "audio.output.stream.channels",
        channels.to_string().as_str(),
    )?;
    config.set(
        "audio.output.stream.sample_rate",
        sample_rate.to_string().as_str(),
    )?;
    config.set(
        "audio.output.stream.buffer_size",
        buffer_size_max.to_string().as_str(),
    )?;
    force_reload();

    debug!("Set output stream to {}", stream);
    Ok(())
}

/// Sets the output buffer size for the stream.
///
/// ### Arguments
///
/// * `buffer_size: u32` - The buffer size to set
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
#[tauri::command]
pub fn set_output_buffer_size(size: u32) -> Result<(), String> {
    let mut config = match OUTPUT_CONFIG.lock() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking OUTPUT_CONFIG: {}", e);
            return Err(format!("Error locking OUTPUT_CONFIG: {}", e));
        }
    };

    let mut config = match config.as_mut() {
        Some(config) => config,
        None => {
            debug!("OUTPUT_CONFIG is None");
            return Err("OUTPUT_CONFIG is None".to_owned());
        }
    };

    config.buffer_size = BufferSize::Fixed(size);

    let mut config = match crate::CONFIG.write() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return Err(format!("Error locking CONFIG: {}", e));
        }
    };

    config.set("audio.output.stream.buffer_size", size.to_string().as_str())?;
    force_reload();

    debug!("Set output buffer size to {}", size);
    Ok(())
}

/// Sets the input stream.
///
/// ### Arguments
///
/// * `stream: String` - The stream to set, in the format "channels sample_rate buffer_size_min-buffer_size_max"
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
#[tauri::command]
pub fn set_input_stream(stream: String) -> Result<(), String> {
    let split = stream
        .split(' ')
        .filter(|e| e.len() > 0)
        .collect::<Vec<_>>();
    if split.len() < 3 {
        return Err(format!("Invalid stream format: {}", stream));
    }
    let channels = split[0].parse::<u32>().unwrap();
    let sample_rate = split[1].parse::<u32>().unwrap();
    let buffer_size = split[2].split('-').collect::<Vec<_>>();
    let buffer_size_min = buffer_size[0].parse::<u32>().unwrap();
    let buffer_size_max = buffer_size[1].parse::<u32>().unwrap();

    let device = match INPUT_DEVICE.lock() {
        Ok(device) => device,
        Err(e) => {
            debug!("Error locking INPUT_DEVICE: {}", e);
            return Err(format!("Error locking INPUT_DEVICE: {}", e));
        }
    };

    let stream_config = get_input_config(
        device.as_ref().unwrap(),
        Preference::Exact(channels, PreferenceAlt::Higher),
        Preference::Exact(sample_rate, PreferenceAlt::Higher),
        Preference::Exact(buffer_size_max, PreferenceAlt::Higher),
    );
    match stream_config {
        Some(stream_config) => {
            let mut config = match INPUT_CONFIG.lock() {
                Ok(config) => config,
                Err(e) => {
                    debug!("Error locking INPUT_CONFIG: {}", e);
                    return Err(format!("Error locking INPUT_CONFIG: {}", e));
                }
            };
            *config = Some(stream_config);
        }
        None => {
            return Err(format!("Could not find input stream {}", stream));
        }
    }

    let mut config = match crate::CONFIG.write() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return Err(format!("Error locking CONFIG: {}", e));
        }
    };

    config.set("audio.input.stream.channels", channels.to_string().as_str())?;
    config.set(
        "audio.input.stream.sample_rate",
        sample_rate.to_string().as_str(),
    )?;
    config.set(
        "audio.input.stream.buffer_size",
        buffer_size_max.to_string().as_str(),
    )?;
    force_reload();

    debug!("Set input stream to {}", stream);
    Ok(())
}

/// Sets the input buffer size for the stream.
///
/// ### Arguments
///
/// * `buffer_size: u32` - The buffer size to set
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
#[tauri::command]
pub fn set_input_buffer_size(size: u32) -> Result<(), String> {
    let mut config = match INPUT_CONFIG.lock() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking INPUT_CONFIG: {}", e);
            return Err(format!("Error locking INPUT_CONFIG: {}", e));
        }
    };

    let mut config = match config.as_mut() {
        Some(config) => config,
        None => {
            debug!("INPUT_CONFIG is None");
            return Err("INPUT_CONFIG is None".to_owned());
        }
    };

    config.buffer_size = BufferSize::Fixed(size);

    let mut config = match crate::CONFIG.write() {
        Ok(config) => config,
        Err(e) => {
            debug!("Error locking CONFIG: {}", e);
            return Err(format!("Error locking CONFIG: {}", e));
        }
    };

    config.set("audio.input.stream.buffer_size", size.to_string().as_str())?;
    force_reload();

    debug!("Set input buffer size to {}", size);
    Ok(())
}

/// ## `reload() -> Result<(), String>`
///
/// Reloads the audio thread.
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
pub fn reload() -> Result<(), String> {
    let mut reload = match RELOAD.write() {
        Ok(reload) => reload,
        Err(e) => {
            debug!("Error locking RELOAD: {}", e);
            return Err(format!("Error locking RELOAD: {}", e));
        }
    };

    *reload = true;

    Ok(())
}

/// ## `audio_thread() -> Result<(), String>`
///
/// Starts the audio thread.
///
/// ### Returns
///
/// * `Result<(), String>` - An error message, or nothing if successful
#[tauri::command]
pub fn audio_thread() -> Result<(), String> {
    // emit event to indicate that the audio thread is starting
    crate::try_emit("updatethread", true);

    let thread = std::thread::spawn(move || {
        let config = {
            match OUTPUT_CONFIG.lock() {
                Ok(config) => match config.as_ref() {
                    Some(config) => config.clone(),
                    None => {
                        debug!("OUTPUT_CONFIG is None");
                        //return Err(format!("OUTPUT_CONFIG is None"));

                        // specify type of Err to avoid type mismatch

                        crate::try_emit("updatethread", false);
                        return Err("OUTPUT_CONFIG is None".to_owned());
                    }
                },
                Err(e) => {
                    debug!("Error locking OUTPUT_CONFIG: {}", e);

                    crate::try_emit("updatethread", false);
                    return Err(format!("Error locking OUTPUT_CONFIG: {}", e));
                }
            }
        };

        let output_stream_opt: Option<Result<cpal::Stream, cpal::BuildStreamError>>;

        {
            let output_device = OUTPUT_DEVICE.lock();
            let output_device = match output_device {
                Ok(output_device) => output_device,
                Err(e) => {
                    debug!("Error locking OUTPUT_DEVICE: {}", e);
                    crate::try_emit("updatethread", false);
                    return Err(format!("Error locking OUTPUT_DEVICE: {}", e));
                }
            };

            let output_device = match output_device.as_ref() {
                Some(output_device) => output_device,
                None => {
                    debug!("OUTPUT_DEVICE is None");
                    crate::try_emit("updatethread", false);
                    return Err("OUTPUT_DEVICE is None".to_owned());
                }
            };

            // Produce a sinusoid of maximum amplitude.
            let mut sample_clock = 0f32;

            let n_channels = config.channels as u32;

            let data_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let buffer_size = data.len();
                let mut strips = match STRIPS.try_write() {
                    Ok(strips) => strips,
                    Err(e) => {
                        debug!("Error locking STRIPS: {}", e);
                        crate::try_emit("updatethread", false);
                        return;
                    }
                };

                let mut channel = 0;

                // cpal audio is interleaved, meaning that every sample is followed by another sample for the next channel
                // example: in a stereo stream, the first sample is for the left channel, the second sample is for the right channel, the third sample is for the left channel, etc.
                // So every other sample is for the same channel
                //
                // So there is a simple formula for determining what channel a sample is for:
                // channel = sample_index % n_channels
                let mut data_vec = Vec::new();
                for sample in data.iter_mut() {
                    if channel % n_channels == 0 {
                        sample_clock += 1.0;
                    }

                    for strip in strips.iter_mut() {
                        match strip.output {
                            Output::Mono(strip_channel) => {
                                if strip_channel == channel % n_channels {
                                    *sample = strip
                                        .process(State {
                                            sample_rate: config.sample_rate.0 as u32,
                                            sample_clock: sample_clock as u64,
                                            buffer_size,
                                        })
                                        .mono();
                                }
                            }
                            Output::Stereo(left_channel, right_channel) => {
                                if left_channel == channel % n_channels {
                                    *sample = strip
                                        .process(State {
                                            sample_rate: config.sample_rate.0 as u32,
                                            sample_clock: sample_clock as u64,
                                            buffer_size,
                                        })
                                        .left();
                                } else if right_channel == channel % n_channels {
                                    *sample = strip
                                        .process(State {
                                            sample_rate: config.sample_rate.0 as u32,
                                            sample_clock: sample_clock as u64,
                                            buffer_size,
                                        })
                                        .right();
                                }
                            }
                            _ => {}
                        }
                    }

                    if channel % n_channels == 0 {
                        data_vec.push(*sample);
                    }
                    channel += 1;
                }

                let tv_window = crate::TV_WINDOW.lock();
                match tv_window {
                    Ok(tv_window) => match tv_window.as_ref() {
                        Some(tv_window) => {
                            let visualizer = <BasicVisualizer as VisualizerTrait>::new();
                            let _ = visualizer.render(tv_window, &data_vec);
                        }
                        None => {
                            debug!("TV_WINDOW is None");
                        }
                    },
                    Err(e) => {
                        debug!("Error locking TV_WINDOW: {}", e);
                    }
                }
            };

            let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
            let output_stream =
                output_device.build_output_stream(&config, data_callback, err_fn, None);
            output_stream_opt = Some(output_stream);
        }

        let output_stream = match output_stream_opt {
            Some(output_stream) => output_stream,
            None => {
                crate::try_emit("updatethread", false);
                return Err("Error building output stream".to_owned());
            }
        };

        let output_stream = match output_stream {
            Ok(stream) => stream,
            Err(err) => {
                crate::try_emit("updatethread", false);
                return Err(format!("Error building output stream: {}", err));
            }
        };

        let _ = output_stream.play();

        let mut reload = false;
        while (!reload) {
            std::thread::sleep(std::time::Duration::from_millis(1000));

            match RELOAD.try_write() {
                Ok(mut r) => {
                    if *r {
                        reload = true;
                        *r = false;
                    }
                }
                Err(_err) => {}
            }
        }

        let _ = output_stream.pause();

        crate::try_emit("updatethread", false);
        let new_thread = audio_thread();
        debug!("Reloading audio thread... {:?}", new_thread);
        Ok(())
    });

    match AUDIO_THREAD.lock() {
        Ok(mut audio_thread) => {
            *audio_thread = Some(thread);
        }
        Err(e) => {
            debug!("Error locking AUDIO_THREAD: {}", e);
            crate::try_emit("updatethread", false);
            return Err(format!("Error locking AUDIO_THREAD: {}", e));
        }
    }
    Ok(())
}

/// ## Sample
///
/// Represents a sample of audio data. Can be either mono or stereo.
///
/// ### Variants
///
/// * `Mono(f32)` - A mono sample
/// * `Stereo(f32, f32)` - A stereo sample
///
/// ### Functions
///
/// * `mono(&self) -> f32` - Returns the mono version of the sample
/// * `stereo(&self) -> (f32, f32)` - Returns the stereo version of the sample
/// * `left(&self) -> f32` - Returns the left channel of the sample
/// * `right(&self) -> f32` - Returns the right channel of the sample
#[derive(Clone, Debug)]
pub enum Sample {
    Mono(f32),
    Stereo(f32, f32),
}

impl Sample {
    /// ## `mono(&self) -> f32`
    ///
    /// Returns the mono version of the sample.
    ///
    /// ### Returns
    ///
    /// * `f32` - The mono version of the sample
    pub fn mono(&self) -> f32 {
        match self {
            Sample::Mono(sample) => *sample,
            Sample::Stereo(left, right) => (*left + *right) / 2.0,
        }
    }

    /// ## `stereo(&self) -> (f32, f32)`
    ///
    /// Returns the stereo version of the sample.
    ///
    /// ### Returns
    ///
    /// * `(f32, f32)` - The stereo version of the sample
    pub fn stereo(&self) -> (f32, f32) {
        match self {
            Sample::Mono(sample) => (*sample, *sample),
            Sample::Stereo(left, right) => (*left, *right),
        }
    }

    /// ## `left(&self) -> f32`
    ///
    /// Returns the left channel of the sample.
    ///
    /// ### Returns
    ///
    /// * `f32` - The left channel of the sample
    pub fn left(&self) -> f32 {
        match self {
            Sample::Mono(sample) => *sample,
            Sample::Stereo(left, _right) => *left,
        }
    }

    /// ## `right(&self) -> f32`
    ///
    /// Returns the right channel of the sample.
    ///
    /// ### Returns
    ///
    /// * `f32` - The right channel of the sample
    pub fn right(&self) -> f32 {
        match self {
            Sample::Mono(sample) => *sample,
            Sample::Stereo(_left, right) => *right,
        }
    }

    /// ## `as_mono(&self) -> Sample`
    ///
    /// Returns the mono version of the sample.
    ///
    /// ### Returns
    ///
    /// * `Sample` - The mono version of the sample
    pub fn as_mono(&self) -> Sample {
        Sample::Mono(self.mono())
    }

    /// ## `as_stereo(&self) -> Sample`
    ///
    /// Returns the stereo version of the sample.
    ///
    /// ### Returns
    ///
    /// * `Sample` - The stereo version of the sample
    pub fn as_stereo(&self) -> Sample {
        Sample::Stereo(self.left(), self.right())
    }
}

/// ## State
///
/// Represents the current state of the audio engine. Primarily configuration settings needed by the effects,
/// and details on what the sample clock is currently at.
///
/// ### Fields
///
/// * `sample_rate: u32` - The sample rate of the audio engine
/// * `sample_clock: u64` - The current sample clock of the audio engine
/// * `buffer_size: usize` - The buffer size of the audio engine
pub struct State {
    pub sample_rate: u32,
    pub sample_clock: u64,
    pub buffer_size: usize,
}

/// ## Output
///
/// Represents an output channel.
///
/// ### Variants
///
/// * `Mono(u32)` - A mono output channel. The u32 represents the output channel number, tied to the interface.
/// * `Stereo(u32, u32)` - A stereo output channel. The u32s represent the left and right output channel numbers, tied to the interface.
/// * `Bus(Box<Input>)` - A bus output channel
pub enum Output {
    Mono(u32),
    Stereo(u32, u32),
    Bus(Arc<Input>),
}

/// ## Input
///
/// Represents an input channel.
///
/// ### Variants
///
/// * `Generator(Box<dyn Generator>)` - A generator input channel
/// * `Bus(Box<Output>)` - A bus input channel
pub enum Input {
    Generator(Arc<Mutex<dyn plugin::Generator>>),
    Bus(Arc<Output>),
}

/// ## Strip
///
/// Represents a strip of audio effects.
///
/// ### Fields
///
/// * `input: Input` - The input channel
/// * `chain: Vec<Box<dyn Effect>>` - The chain of effects
/// * `output: Output` - The output channel
///
/// ### Functions
///
/// * `new(input: Input, output: Output) -> Self` - Creates a new strip
/// * `add_effect(&mut self, effect: Box<dyn Effect>)` - Adds an effect to the end of the chain
/// * `insert_effect(&mut self, effect: Box<dyn Effect>, index: usize)` - Inserts an effect into the chain at the given index
/// * `remove_effect(&mut self, index: usize)` - Removes an effect from the chain at the given index
/// * `process(&mut self, state: State) -> Sample` - Processes a sample
pub struct Strip {
    input: Input,
    chain: Vec<Box<dyn plugin::Effect>>,
    output: Output,
}

impl Strip {
    /// ## `new(input: Input, output: Output) -> Self`
    ///
    /// Creates a new strip.
    ///
    /// ### Arguments
    ///
    /// * `input: Input` - The input channel
    /// * `output: Output` - The output channel
    ///
    /// ### Returns
    ///
    /// * `Self` - The new strip
    pub fn new(input: Input, output: Output) -> Self {
        Self {
            input,
            chain: Vec::new(),
            output,
        }
    }

    /// ## `add_effect(&mut self, effect: Box<dyn Effect>)`
    ///
    /// Adds an effect to the end of the chain.
    ///
    /// ### Arguments
    ///
    /// * `effect: Box<dyn Effect>` - The effect to add
    pub fn add_effect(&mut self, effect: Box<dyn plugin::Effect>) {
        self.chain.push(effect);
    }

    /// ## `insert_effect(&mut self, effect: Box<dyn Effect>, index: usize)`
    ///
    /// Inserts an effect into the chain at the given index.
    ///
    /// ### Arguments
    ///
    /// * `effect: Box<dyn Effect>` - The effect to insert
    /// * `index: usize` - The index to insert the effect at
    pub fn insert_effect(&mut self, effect: Box<dyn plugin::Effect>, index: usize) {
        self.chain.insert(index, effect);
    }

    /// ## `remove_effect(&mut self, index: usize)`
    ///
    /// Removes an effect from the chain at the given index.
    ///
    /// ### Arguments
    ///
    /// * `index: usize` - The index to remove the effect from
    pub fn remove_effect(&mut self, index: usize) {
        self.chain.remove(index);
    }

    /// ## `process(&mut self, state: State) -> Sample`
    ///
    /// Processes a sample.
    ///
    /// ### Arguments
    ///
    /// * `state: State` - The current state of the audio engine
    ///
    /// ### Returns
    ///
    /// * `Sample` - The resulting sample
    pub fn process(&mut self, state: State) -> Sample {
        let sample = match &self.input {
            Input::Generator(generator) => {
                let mut sample = match generator.try_lock() {
                    Ok(mut generator) => generator.generate(&state),
                    Err(error) => return Sample::Mono(0.0),
                };
                for effect in self.chain.iter_mut() {
                    effect.process(&state, &mut sample);
                }
                sample
            }
            Input::Bus(_bus) => Sample::Mono(0.0),
        };

        match &self.output {
            Output::Mono(_channel) => Sample::Mono(sample.mono()),
            Output::Stereo(_left_channel, _right_channel) => {
                Sample::Stereo(sample.left(), sample.right())
            }
            Output::Bus(_bus) => Sample::Stereo(sample.left(), sample.right()),
        }
    }

	pub fn to_js(&self) -> serde_json::Value {
		let input = match self.input {
			Input::Generator(ref generator) => {
				match generator.as_ref().try_lock() {
					Ok(generator) => {
						generator.name().to_string()
					}
					Err(_) => {
						"invalid (generator)".to_string()
					}
				}
			}
			Input::Bus(ref bus) => {
				match bus.as_ref() {
					Output::Mono(channel) => {
						"invalid (mono)".to_string()
					}
					Output::Stereo(left_channel, right_channel) => {
						"invalid (stereo)".to_string()
					}
					Output::Bus(_) => {
						format!("bus")
					}
				}
			}
		};
		let output = match self.output {
			Output::Mono(channel) => {
				format!("mono({})", channel)
			}
			Output::Stereo(left_channel, right_channel) => {
				format!("stereo({}, {})", left_channel, right_channel)
			}
			Output::Bus(_) => {
				format!("bus")
			}
		};

		let mut chain = Vec::new();
		for effect in self.chain.iter() {
			chain.push(effect.name());
		}

		serde_json::json!({
			"input": input,
			"chain": chain,
			"output": output,
		})
	}
}

pub fn map_strips() {
	match STRIPS.read() {
		Ok(strips) => {
			crate::try_emit("rust-clearstrips", ());
			for (index, strip) in strips.iter().enumerate() {
				crate::try_emit("rust-updatestrip", strip.to_js());
			}
		}
		Err(e) => {
			debug!("Error locking STRIPS: {}", e);
		}
	}
}

pub fn remove_strip(index: usize) {
	match STRIPS.write() {
		Ok(mut strips) => {
			crate::try_emit("rust-removestrip", index);
			strips.remove(index);
		}
		Err(e) => {
			debug!("Error locking STRIPS: {}", e);
		}
	}
}

pub fn add_strip(strip: Strip) -> Option<usize> {
	match STRIPS.write() {
		Ok(mut strips) => {
			crate::try_emit("rust-updatestrip", strip.to_js());
			strips.push(strip);
			Some(strips.len() - 1)
		}
		Err(e) => {
			debug!("Error locking STRIPS: {}", e);
			return None;
		}
	}
}

#[tauri::command]
pub fn play_sample(path: &str) {
	let mut played = false;
	{
	let mut strips = match STRIPS.write() {
		Ok(strips) => strips,
		Err(e) => {
			debug!("Error locking STRIPS: {}", e);
			return;
		}
	};

	
	for strip in strips.iter_mut() {
		match strip {
			Strip {
				input: Input::Generator(ref generator),
				..
			} => {
				match generator.as_ref().try_lock() {
					Ok(mut generator) => {
						let command_a = Command::Multiple(SampleGenerator::SET_SAMPLE, vec![Command::String(path.to_string())]);
						let command_b = Command::Single(SampleGenerator::PLAY_SAMPLE);
						let _ = generator.command(command_a);
						let _ = generator.command(command_b);
						played = true;
					}
					Err(_) => {}
				}
			}
			_ => {}
		}
	}
	}
	if !played {
		let sample_generator = SampleGenerator::new(path);
		let strip = Strip::new(
			Input::Generator(Arc::new(Mutex::new(sample_generator))),
			Output::Stereo(0, 1),
		);
		let _ = add_strip(strip);
		play_sample(path);
	}
}

pub fn listen_frontend() -> Result<(), String> {
	let app = {
		match crate::APP_HANDLE.lock() {
			Ok(app) => match app.as_ref() {
				Some(app) => app.clone(),
				None => {
					debug!("APP_HANDLE is None");
					return Err("APP_HANDLE is None".to_owned());
				}
			},
			Err(e) => {
				debug!("Error locking APP_HANDLE: {}", e);
				return Err(format!("Error locking APP_HANDLE: {}", e));
			}
		}
	};

	app.listen_global("svelte-updatestrip", | event | {
		debug!("Received svelte-updatestrip event");

		let payload: serde_json::Value = serde_json::from_str(event.payload().unwrap()).unwrap();
		let index = payload["index"].as_u64().unwrap() as usize;
		let kind = payload["kind"].as_str().unwrap();
		match kind {
			"output-mono" => {
				let channel = payload["channel"].as_u64().unwrap() as u32;
				match STRIPS.write() {
					Ok(mut strips) => {
						match strips.get_mut(index) {
							Some(strip) => {
								debug!("Setting output to mono {}", channel);
								strip.output = Output::Mono(channel);
							}
							None => {}
						}
					}
					Err(e) => {
						debug!("Error locking STRIPS: {}", e);
					}
				}
			}
			"output-stereo" => {
				let left_channel = payload["left"].as_u64().unwrap() as u32;
				let right_channel = payload["right"].as_u64().unwrap() as u32;
				match STRIPS.write() {
					Ok(mut strips) => {
						match strips.get_mut(index) {
							Some(strip) => {
								debug!("Setting output to stereo {} {}", left_channel, right_channel);
								strip.output = Output::Stereo(left_channel, right_channel);
							}
							None => {}
						}
					}
					Err(e) => {
						debug!("Error locking STRIPS: {}", e);
					}
				}
			}
			_ => {}
		}
	});

	Ok(())
}