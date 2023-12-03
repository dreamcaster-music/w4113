//! ## config.rs
//!
//! This module is used for anything related to configuration and in the filesystem.

use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, RwLockReadGuard, RwLockWriteGuard};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

lazy_static::lazy_static! {
    static ref APP: Mutex<Option<AppHandle>> = Mutex::new(None);
}

use log::{error, debug};
use tauri::{AppHandle, Manager};

/// Used to serialize/deserialize updates the config and make it easier to communicate with the frontend.
/// 
/// ### Fields
/// 
/// * `key` - The key of the setting.
/// * `value` - The value of the setting.
#[derive(ts_rs::TS, Clone, serde::Serialize, serde::Deserialize)]
#[ts(export, export_to = "../src/bindings/ConfigUpdate.ts")]
struct Update {
	key: String,
	value: String,
}

/// Sets the Tauri app handle where events will be sent to the frontend.
/// 
/// ### Arguments
/// 
/// * `app: AppHandle` - The Tauri app handle.
pub fn set_app(app: AppHandle) {
    let mut app_mutex = APP.lock().unwrap();
    *app_mutex = Some(app);
}

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
struct Setting {
    key: String,
    value: String,
    on_change: Option<Box<dyn Fn(&str, &str) + Send + Sync + 'static>>,
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
        match APP.lock() {
            Ok(app) => match app.as_ref() {
                Some(app) => {
                    let payload = Update {
						key: self.key.clone(),
						value: self.value.clone(),
					};
                    let emit = app.emit_all("rust-state-update", payload);
                    match emit {
                        Ok(_) => {}
                        Err(err) => {
                            error!("{}", err);
                        }
                    }
                }
                None => {}
            },
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
    fn when_changed(&mut self, function: impl Fn(&str, &str) + Send + Sync + 'static) {
        self.on_change = Some(Box::new(function));
    }
}

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
fn config_json_get_or_create(path: &str) -> Result<serde_json::Value, String> {
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
    /// Creates an empty config.
    pub fn empty() -> Config {
        Config {
            path: "".to_string(),
            saved: true,
            settings: HashMap::new(),
            json: serde_json::Value::Null,
        }
    }

    ///	Starts a thread that listens for changes to the config.
    ///
    /// ### Arguments
    ///
    /// * `config: Arc<RwLock<Config>>` - The config.
    pub fn listen(config: Arc<RwLock<Config>>) {
        let _thread = std::thread::spawn(move || {
            loop {
                match APP.lock() {
                    Ok(app) => match app.as_ref() {
                        Some(app) => {
                            break;
                        }
                        None => {}
                    },
                    Err(err) => {
                        error!("{}", err);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }

            let app = APP.lock().unwrap().as_ref().unwrap().clone();
            let config = config.clone();
            app.listen_global("react-state-update", move |event| {
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

                let _ = config.set(&key, &value);
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
    /// * `Result<Config, String>` - The config.
    pub fn load(path: &str) -> Result<Config, String> {
        let json = config_json_get_or_create(&path);

        match json {
            Ok(json) => Ok(Config {
                path: path.to_string(),
                saved: true,
                settings: HashMap::new(),
                json: json,
            }),
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
    fn translate_mut(&mut self, string_value: &str) -> Result<&mut serde_json::Value, String> {
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

    fn translate(&self, string_value: &str) -> Result<&serde_json::Value, String> {
        let string_value = string_value.to_string();
        let split: Vec<&str> = string_value.split(".").collect::<Vec<&str>>();

        let mut value = &self.json;

        for i in 0..split.len() {
            let key = split[i];
            let _temp = match value[key] {
                _ => {
                    value = &value[key];
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
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        {
            let setting = self.settings.get_mut(key);
            match setting {
                Some(setting) => {
					setting.set_value(value.to_string());
                    //setting.changed();
                }
                None => {
                    let setting = Setting {
                        key: key.to_string(),
                        value: value.to_string(),
                        on_change: None,
                    };
                    self.settings.insert(key.to_string(), setting);
                }
            }
        }

        let json = self.translate_mut(key);

        match json {
            Ok(json) => {
                *json = serde_json::Value::String(value.to_string());
                self.saved = false;
                Ok(())
            }
            Err(err) => {
                let err = format!(
                    "An error occurred while setting config key {}: {}",
                    key, err
                );
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
    pub fn get_or(&mut self, key: &str, or: fn() -> String) -> Result<String, String> {
        let json = self.translate(key);

        let json_value = match json {
            Ok(json) => json.clone(),
            Err(err) => {
                let err = format!(
                    "An error occurred while getting config key {}: {}",
                    key, err
                );
                error!("{}", err);
                return Err(err);
            }
        };

        let string_value = if json_value.is_string() {
            let string_value = json_value.as_str();
            match string_value {
                Some(string_value) => string_value.to_string(),
                None => {
                    let value = or();
                    self.set(key, &value)?;
                    value
                }
            }
        } else {
            let value = or();
            self.set(key, &value)?;
            value
        };

        return Ok(string_value);
    }

    /// Sets the on_change function for a setting.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config.
    /// * `key: String` - The key of the setting.
    /// * `function: impl Fn(&String, &String) + Send + Sync + 'static` - The function to call when the setting is changed.
    pub fn when_changed(
        &mut self,
        key: &str,
        function: impl Fn(&str, &str) + Send + Sync + 'static,
    ) {
        let setting = self.settings.get_mut(key);
        match setting {
            Some(setting) => {
                setting.when_changed(function);
            }
            None => {
                let mut setting = Setting {
                    key: key.to_string(),
                    value: "".to_string(),
                    on_change: None,
                };
                setting.when_changed(function);
                self.settings.insert(key.to_string(), setting);
            }
        }
    }

    /// Returns whether or not the config has been saved.
    ///
    /// ### Arguments
    ///
    /// * `&self` - The config.
    ///
    /// ### Returns
    ///
    /// * `bool` - Whether or not the config has been saved.
    pub fn saved(&self) -> bool {
        self.saved
    }

    /// Returns the path to the config file.
    ///
    /// ### Arguments
    ///
    /// * `&self` - The config.
    ///
    /// ### Returns
    ///
    ///	* `String` - The path to the config file.
    pub fn path(&self) -> String {
        self.path.to_string()
    }

    /// Returns the json value of the config.
    ///
    /// ### Arguments
    ///
    /// * `&self` - The config.
    ///
    /// ### Returns
    ///
    /// * `&serde_json::Value` - The json value of the config.
    pub fn json(&self) -> &serde_json::Value {
        &self.json
    }
}
