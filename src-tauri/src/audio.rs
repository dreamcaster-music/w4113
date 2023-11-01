//! ## Audio.rs
//!
//! This file contains the audio system.
//!
//! ### Structs
//!
//! * `Properties` - Contains all the audio properties needed to manage the audio system.
//!
//! ### Methods
//!
//! * `Properties::empty() -> AudioProperties` - Create an empty audio properties struct
//! * `Properties::from_config(config: &Config) -> Result<AudioProperties, String>` - Create an audio properties struct from the given config

use cpal::traits::{DeviceTrait, HostTrait};
use log::debug;

use crate::config::Config;

/// ## Properties
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
/// * `empty() -> Properties` - Create an empty audio properties struct
/// * `from_config(config: &Config) -> Result<Properties, String>` - Create an audio properties struct from the given config
pub struct Properties {
    host: Option<cpal::Host>,
    output_device: Option<cpal::Device>,
    input_device: Option<cpal::Device>,
    output_streams: Vec<cpal::Stream>,
    input_streams: Vec<cpal::Stream>,
}

impl Properties {
    /// ## empty() -> Properties
    ///
    /// Create an empty audio properties struct
    ///
    /// ### Returns
    ///
    /// * `Properties` - The empty audio properties struct
    pub fn empty() -> Properties {
        Properties {
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
    pub fn from_config(config: &Config) -> Result<Properties, String> {
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
                    });

                let device = match device {
                    Some(device) => device,
                    None => {
                        let device = host
                            .default_output_device()
                            .ok_or("Could not find default output device")?;
                        debug!("Using default output device {}", device.name().unwrap());
                        device
                    }
                };
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
                    });
                let device = match device {
                    Some(device) => device,
                    None => {
                        let device = host
                            .default_input_device()
                            .ok_or("Could not find default input device")?;
                        debug!("Using default input device {}", device.name().unwrap());
                        device
                    }
                };
                Some(device)
            }
            None => None,
        };

        let output_streams = Vec::new();
        let input_streams = Vec::new();

        Ok(Properties {
            host: Some(host),
            output_device,
            input_device,
            output_streams,
            input_streams,
        })
    }
}
