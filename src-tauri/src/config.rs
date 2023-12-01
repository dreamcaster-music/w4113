//! ## config.rs
//!
//! This module is used for anything related to configuration and in the filesystem.

use std::{
    ops::Deref,
    sync::{Arc, RwLock}, string, collections::HashMap,
};

use log::{debug, error, trace};
use tauri::Manager;

/// An individual setting within a config.
/// 
/// ### Fields
/// 
/// * `key` - The key of the setting.
/// * `value` - The value of the setting.
/// * `on_change` - The function to call when the setting is changed.
/// 
/// ### Methods
/// 
/// * `key(&self) -> &String` - Returns the key of the setting.
/// * `value(&self) -> &String` - Returns the value of the setting.
/// * `set_value(&mut self, value: String)` - Sets the value of the setting.
/// * `changed(&self)` - Calls the on_change function.
/// * `when_changed(&mut self, function: impl Fn(&String, &String) + Send + Sync + 'static)` - Sets the on_change function.
pub struct Setting {
	key: String,
	value: String,
	on_change: Option<Box<dyn Fn(&String, &String) + Send + Sync + 'static>>,
}

impl Setting {
	/// Returns the key of the setting.
	/// 
	/// ### Arguments
	/// 
	/// * `&self` - The setting.
	/// 
	/// ### Returns
	/// 
	/// * `&String` - The key of the setting.
	fn key(&self) -> &String {
        &self.key
    }

	/// Returns the value of the setting.
	/// 
	/// ### Arguments
	/// 
	/// * `&self` - The setting.
	/// 
	/// ### Returns
	/// 
	/// * `&String` - The value of the setting.
    fn value(&self) -> &String {
        &self.value
    }

	/// Sets the value of the setting.
	/// 
	/// ### Arguments
	/// 
	/// * `&mut self` - The setting.
	/// * `value: String` - The value to set.
    fn set_value(&mut self, value: String) {
        if value != self.value {
            self.value = value;
            match &self.on_change {
                Some(function) => {
                    function.as_ref()(&self.key, &self.value);
                }
                None => {}
            }
        }
    }

	/// Calls the on_change function.
	/// 
	/// ### Arguments
	/// 
	/// * `&self` - The setting.
	fn changed(&self) {
		match &self.on_change {
			Some(function) => {
				function.as_ref()(&self.key, &self.value);
			}
			None => {}
		}
		match crate::APP.lock() {
			Ok(app) => {
				match app.as_ref() {
					Some(app) => {
						let payload = format!("{{\"key\": \"{}\", \"value\": \"{}\"}}", self.key, self.value);
						let emit = app.emit_all("rust-state-update", payload);
						match emit {
							Ok(_) => {}
							Err(err) => {
								error!("{}", err);
							}
						}
					}
					None => {}
				}
			}
			Err(err) => {
				error!("{}", err);
			}
		}
	}

	/// Sets the on_change function for the setting.
	/// 
	/// ### Arguments
	/// 
	/// * `&mut self` - The setting.
	/// * `function: impl Fn(&String, &String) + Send + Sync + 'static` - The function to call when the setting is changed.
    fn when_changed(&mut self, function: impl Fn(&String, &String) + Send + Sync + 'static) {
        self.on_change = Some(Box::new(function));
    }
}

type ArcConfig = Arc<RwLock<Config>>;

/// The config struct.
/// 
/// ### Fields
/// 
/// * `path` - The path to the config file.
/// * `saved` - Whether or not the config has been saved.
/// * `settings` - A hashmap of settings.
///	* `json` - The json value of the config.
/// 
/// ### Methods
/// 
/// * `listen(config: ArcConfig)` - Listens for changes to the config.
/// * `load(path: String) -> Result<ArcConfig, String>` - Loads the config.
/// * `save(&mut self) -> Result<(), String>` - Saves the config.
/// * `translate(&mut self, string_value: &str) -> Result<&mut serde_json::Value, String>` - Translates a string value to a json value.
/// * `set(&mut self, key: String, value: String) -> Result<(), String>` - Sets a value in the config.
/// * `get_or(&mut self, key: String, or: Box<dyn Fn() -> String>) -> Result<String, String>` - Gets a value from the config or returns a default value.
/// * `when_changed(&mut self, key: String, function: impl Fn(&String, &String) + Send + Sync + 'static)` - Sets the on_change function for a setting.
pub struct Config {
	path: String,
	saved: bool,
    settings: HashMap<String, Setting>,
    json: serde_json::Value,
}

/// Gets the json value of the config or creates the config file if it doesn't exist.
/// 
/// ### Arguments
/// 
/// * `path: &String` - The path to the config file.
/// 
/// ### Returns
/// 
/// * `Result<serde_json::Value, String>` - The json value of the config.
fn config_json_get_or_create(path: &String) -> Result<serde_json::Value, String> {
		// Create file if it doesn't exist
		if !std::path::Path::new(path).exists() {
			let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
			serde_json::to_writer_pretty(file, &serde_json::Value::Null).map_err(|e| e.to_string())?;
		}

		let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
		let json = serde_json::from_reader(file).map_err(|e| e.to_string())?;
		return Ok(json);
}

impl Config {
	///	Starts a thread that listens for changes to the config.
	/// 
	/// ### Arguments
	/// 
	/// * `config: ArcConfig` - The config.
	fn listen(config: ArcConfig) {
		let thread = std::thread::spawn(move || {
			loop {
				match crate::APP.lock() {
					Ok(app) => {
						match app.as_ref() {
							Some(app) => {
								break;
							}
							None => {}
						}
					}
					Err(err) => {
						error!("{}", err);
					}
				}
				std::thread::sleep(std::time::Duration::from_millis(1000));
			}

			let app = crate::APP.lock().unwrap().as_ref().unwrap().clone();
			let config = config.clone();
			app.listen_global("react-state-update", move | event| {
				let mut config = match config.write() {
					Ok(config) => config,
					Err(err) => {
						error!("{}", err);
						return;
					}
				};

				let payload = match event.payload() {
					Some(payload) => payload,
					None => {
						return;
					}
				};

				let json = match serde_json::from_str::<serde_json::Value>(payload) {
					Ok(json) => json,
					Err(err) => {
						error!("{}", err);
						return;
					}
				};

				let key = match json["key"].as_str() {
					Some(key) => key.to_string(),
					None => {
						return;
					}
				};

				let value = match json["value"].as_str() {
					Some(value) => value.to_string(),
					None => {
						return;
					}
				};

				{
					let json = config.translate(key.as_str());
						match json {
							Ok(json) => {
								*json = serde_json::Value::String(value.clone());
							}
							Err(err) => {
								error!("{}", err);
							}
						}
				}

				match config.settings.get_mut(&key) {
					Some(setting) => {
						setting.set_value(value.to_string());
						setting.changed();
					}
					None => {
						let setting = Setting {
							key: key.clone(),
							value: value.clone(),
							on_change: None,
						};
						config.settings.insert(key.clone(), setting);
					}
				}
			});
		});
	}

	/// Loads the config.
	/// 
	/// ### Arguments
	/// 
	/// * `path: String` - The path to the config file.
	/// 
	/// ### Returns
	/// 
	/// * `Result<ArcConfig, String>` - The config.
	pub fn load(path: String) -> Result<ArcConfig, String> {
		let json = config_json_get_or_create(&path);

		match json {
			Ok(json) => Ok(Arc::new(RwLock::new(Config {
				path,
				saved: true,
				settings: HashMap::new(),
				json: json,
			}))),
			Err(err) => {
				let err = format!("An error occurred while loading config {}", err);
				error!("{}", err);
				Err(err)
			}
		}
	}

	/// Saves the config.
	/// 
	/// ### Arguments
	/// 
	/// * `&mut self` - The config.
	/// 
	/// ### Returns
	/// 
	/// * `Result<(), String>` - Whether or not the config was saved.
	pub fn save(&mut self) -> Result<(), String> {
		let file = std::fs::File::create(&self.path).map_err(|e| e.to_string())?;
        serde_json::to_writer_pretty(file, &self.json).map_err(|e| e.to_string())?;
		self.saved = true;
		Ok(())
	}

	/// Translates a string value to a json value.
	/// 
	/// ### Arguments
	/// 
	/// * `&mut self` - The config.
	/// * `string_value: &str` - The string value.
	/// 
	/// ### Returns
	/// 
	/// * `Result<&mut serde_json::Value, String>` - The json value.
	fn translate(&mut self, string_value: &str) -> Result<&mut serde_json::Value, String> {
		let string_value = string_value.to_string();
		let split: Vec<&str> = string_value.split(".").collect::<Vec<&str>>();

		let mut value = &mut self.json;

		for i in 0..split.len() {
			let key = split[i];
			let _temp = match value[key] {
				_ => {
					value = &mut value[key];
				}
			};
		}

		Ok(value)
	}

	/// Sets a value in the config.
	/// 
	/// ### Arguments
	/// 
	/// * `&mut self` - The config.
	/// * `key: String` - The key of the setting.
	/// * `value: String` - The value of the setting.
	/// 
	/// ### Returns
	/// 
	/// * `Result<(), String>` - Whether or not the value was set.
	pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
		{
			let setting = self.settings.get_mut(&key);
			match setting {
				Some(setting) => {
					setting.set_value(value.clone());
					setting.changed();
				}
				None => {
					let setting = Setting {
						key: key.clone(),
						value: value.clone(),
						on_change: None,
					};
					self.settings.insert(key.clone(), setting);
				}
			}
		}

		let json = self.translate(key.as_str());

		match json {
			Ok(json) => {
				*json = serde_json::Value::String(value);
				self.saved = false;
				Ok(())
			}
			Err(err) => {
				let err = format!("An error occurred while setting config key {}: {}", key, err);
				error!("{}", err);
				return Err(err);
			}
		}
	}

	/// Gets a value from the config or returns a default value.
	/// 
	/// ### Arguments
	/// 
	/// * `&mut self` - The config.
	/// * `key: String` - The key of the setting.
	/// * `or: Box<dyn Fn() -> String>` - The function to call if the value doesn't exist.
	/// 
	/// ### Returns
	/// 
	/// * `Result<String, String>` - The value of the setting.
	pub fn get_or(&mut self, key: String, or: Box<dyn Fn() -> String>) -> Result<String, String> {
		let json = self.translate(key.as_str());
		match json {
			Ok(json) => {
				let string_value = if json.is_string() {
					let string_value = json.as_str();
					match string_value {
						Some(string_value) => {
							string_value.to_string()
						}
						None => {
							or()
						}
					}
				} else {
					or()
				};

				*json = serde_json::Value::String(string_value.clone());
				return Ok(string_value);
			}
			Err(err) => {
				let err = format!("An error occurred while getting config key {}: {}", key, err);
				error!("{}", err);
				return Err(err);
			}
		}
	}

	/// Sets the on_change function for a setting.
	/// 
	/// ### Arguments
	/// 
	/// * `&mut self` - The config.
	/// * `key: String` - The key of the setting.
	/// * `function: impl Fn(&String, &String) + Send + Sync + 'static` - The function to call when the setting is changed.
	pub fn when_changed(&mut self, key: String, function: impl Fn(&String, &String) + Send + Sync + 'static) {
		let setting = self.settings.get_mut(&key);
		match setting {
			Some(setting) => {
				setting.when_changed(function);
			}
			None => {
				let mut setting = Setting {
					key: key.clone(),
					value: "".to_string(),
					on_change: None,
				};
				setting.when_changed(function);
				self.settings.insert(key.clone(), setting);
			}
		}
	}
}