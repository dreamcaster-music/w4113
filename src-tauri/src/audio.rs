//! audio.rs
//!
//! Module is used for interacting with audio drivers/hardware

use std::sync::Mutex;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, Device, Host, StreamConfig,
    SupportedStreamConfigRange,
};
use lazy_static::lazy_static;
use log::debug;

use crate::tv::VisualizerTrait;

lazy_static! {
    pub static ref HOST: Mutex<Option<cpal::Host>> = Mutex::new(None);
    pub static ref OUTPUT_DEVICE: Mutex<Option<cpal::Device>> = Mutex::new(None);
    pub static ref INPUT_DEVICE: Mutex<Option<cpal::Device>> = Mutex::new(None);
    pub static ref OUTPUT_CONFIG: Mutex<Option<cpal::StreamConfig>> = Mutex::new(None);
    pub static ref INPUT_CONFIG: Mutex<Option<cpal::StreamConfig>> = Mutex::new(None);
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
        debug!("Returning default output device {}", default_device_name);
        return default_device;
    }

    let devices = host.output_devices();

    let devices = match devices {
        Ok(devices) => devices,
        Err(_err) => {
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
            Err(_err) => {
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
        debug!("Returning default input device {}", default_device_name);
        return default_device;
    }

    let devices = host.input_devices();

    let devices = match devices {
        Ok(devices) => devices,
        Err(_err) => {
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
            Err(_err) => {
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

/// ## `list_hosts() -> Vec<String>`
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
pub fn list_output_streams(device: &Device) -> Result<Vec<String>, String> {
    let supported_configs = match device.supported_output_configs() {
        Ok(supported_configs) => supported_configs,
        Err(err) => {
            return Err(format!("Error getting supported output configs: {}", err));
        }
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
            "{} Channels, {} Hz, {}-{} Buffer Size",
            channels, sample_rate, buffer_size.0, buffer_size.1
        );
        streams.push(stream);
    }

    Ok(streams)
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
pub fn list_input_streams(device: &Device) -> Result<Vec<String>, String> {
    let supported_configs = match device.supported_input_configs() {
        Ok(supported_configs) => supported_configs,
        Err(err) => {
            return Err(format!("Error getting supported input configs: {}", err));
        }
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
            "{} Channels, {} Samples, {}-{} Buffer Size",
            channels, sample_rate, buffer_size.0, buffer_size.1
        );
        streams.push(stream);
    }

    Ok(streams)
}

pub fn sine(
    window: tauri::Window,
    config: &StreamConfig,
    freq: f32,
    amp: f32,
    dur: f32,
) -> Result<(), String> {
    let output_stream_opt: Option<Result<cpal::Stream, cpal::BuildStreamError>>;

	// here we create a new scope so that the mutex is unlocked before we try to play the sound
    {
        let output_device = OUTPUT_DEVICE.try_lock();
        let output_device = match output_device {
            Ok(output_device) => output_device,
            Err(e) => {
                debug!("Error locking OUTPUT_DEVICE: {}", e);
                return Err(format!("Error locking OUTPUT_DEVICE: {}", e));
            }
        };

        let output_device = match output_device.as_ref() {
            Some(output_device) => output_device,
            None => {
                debug!("OUTPUT_DEVICE is None");
                return Err("OUTPUT_DEVICE is None".to_owned());
            }
        };
        debug!(
            "Playing sine wave with frequency {} Hz, amplitude {}, and duration {} seconds...",
            freq, amp, dur
        );

        let sample_rate = config.sample_rate.0 as f32;

        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;

        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * freq * std::f32::consts::PI / sample_rate).sin()
        };

        let data_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let buffer_size = data.len();
            debug!("Buffer size is cromulating at len: {}", buffer_size);
            let visualizer = crate::tv::BasicVisualizer::new();

            let mut channel = 0;
            // cpal audio is interleaved, meaning that every sample is followed by another sample for the next channel
            // example: in a stereo stream, the first sample is for the left channel, the second sample is for the right channel, the third sample is for the left channel, etc.
            // So every other sample is for the same channel
            //
            // So there is a simple formula for determining what channel a sample is for:
            // channel = sample_index % n_channels
            for sample in data.iter_mut() {
                if channel % 2 == 0 {
                    *sample = next_value() * amp;
                } else {
                    *sample = 0.0;
                }
                channel += 1;
            }

            let _ = visualizer.render(&window, data);
        };
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let output_stream = output_device.build_output_stream(&config, data_callback, err_fn, None);

        output_stream_opt = Some(output_stream);
    }

    let output_stream = output_stream_opt.unwrap();

    match output_stream {
        Ok(stream) => {
            let _ = stream.play();
            std::thread::sleep(std::time::Duration::from_secs_f32(dur));
            stream.pause().unwrap();
        }
        Err(err) => {
            return Err(format!("Error building output stream: {}", err));
        }
    };

    Ok(())
}

/*

This is code that will be used in the future

/// ## Channel
///
/// Used to indicate the direction and channel ID of a channel.
///
/// ### Variants
///
/// * `Input(u16)` - The channel is an input channel with the given ID
/// * `Output(u16)` - The channel is an output channel with the given ID
/// * `None` - The channel is not an input or output channel. Essentially "Unknown" or "Error" since this should never occur
struct Channel {
    direction: Direction,
    channel_id: u16,
}

/// ## Direction
///
/// Used to indicate the direction of a channel.
///
/// ### Variants
///
/// * `Input` - The channel is an input channel
/// * `Output` - The channel is an output channel
/// * `None` - The channel is not an input or output channel. Essentially "Unknown" or "Error" since this should never occur
enum Direction {
    Input,
    Output,
    None,
}

/// ## Channels
///
/// Used to indicate the channels of a device.
///
/// ### Variants
///
/// * `Mono(Channel)` - The device has one channel
/// * `Stereo(Channel, Channel)` - The device has two channels
/// * `Custom(u16)` - The device has a custom number of channels
enum Channels {
    Mono(Channel),
    Stereo(Channel, Channel),
    Custom(u16),
}
*/
