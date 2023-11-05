//! ## config.rs
//!
//! This module is used for anything related to configuration and in the filesystem.

use log::debug;
use serde_json::Value;

/// ## State
///
/// Represents the state of a config.
///
/// ### Variants
///
/// * `Unsaved` - The config has been modified since it was last saved
/// * `Saved` - The config has not been modified since it was last saved
#[derive(Clone)]
pub enum State {
    Unsaved,
    Saved,
}

/// ## Config
///
/// Represents a config.
///
/// ### Fields
///
/// * `state: State` - The state of the config
/// * `json: serde_json::Value` - The JSON value of the config
///
/// ### Functions
///
/// * `state(&self) -> &State` - Returns the state of the config
/// * `set(&mut self, key: &str, value: &str)` - Sets a key in the config
/// * `get_or(&mut self, key: &str, default: impl Fn() -> String) -> Result<String, String>` - Gets a key from the config, or sets it to a default value if it doesn't exist
/// * `empty() -> Self` - Creates an empty config
/// * `load_from_file(path: &str) -> Result<Self, String>` - Loads a config from a file
/// * `save_to_file(&mut self, path: &str) -> Result<(), String>` - Saves a config to a file
#[derive(Clone)]
pub struct Config {
    state: State,
    json: serde_json::Value,
}

impl Config {
    /// ## translate(&mut self, string_value: &str) -> Result<&mut serde_json::Value, String>
    ///
    /// Private function which translates a string value into a JSON value.
    /// JSON values are typically accessed by using a string value, such as "key.subkey.subsubkey",
    /// but serde_json::Value does not support this, it does it by calling it like this:
    /// ```
    /// let mut config = config::load_from_file("config.json");
    /// let value = config.json["key"]["subkey"]["subsubkey"];
    /// ```
    /// This function allows you to translate a string value into a JSON value that can be used by serde_json::Value.
    ///
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `string_value: &str` - The string value
    ///
    /// ### Returns
    ///
    /// * `Ok(&mut serde_json::Value)` - The JSON value
    /// * `Err(String)` - The result of the command
    ///
    /// ### Examples
    ///
    /// ```
    /// let mut config = config::load_from_file("config.json");
    /// let value = config.translate("key.subkey.subsubkey");
    /// ```
    fn translate(&mut self, string_value: &str) -> Result<&mut serde_json::Value, String> {
        let string_value = string_value.to_string();
        let split: Vec<&str> = string_value.split(".").collect::<Vec<&str>>();

        let mut value = &mut self.json;

        for i in 0..split.len() {
            let key = split[i];
            match value[key] {
                _ => {
                    value = &mut value[key];
                }
            }
        }

        Ok(value)
    }

    /// ## state(&self) -> &State
    ///
    /// Returns the state of the config.
    ///
    /// ### Arguments
    ///
    /// * `&self` - The config
    ///
    /// ### Returns
    ///
    /// * `&State` - The state of the config
    pub fn state(&self) -> &State {
        &self.state
    }

    /// ## json(&self) -> &serde_json::Value
    ///
    /// Returns the JSON value of the config.
    ///
    /// ### Arguments
    ///
    /// * `&self` - The config
    ///
    /// ### Returns
    ///
    /// * `&serde_json::Value` - The JSON value of the config
    pub fn json(&self) -> &serde_json::Value {
        &self.json
    }

    /// ## set(&mut self, key: &str, value: &str)
    ///
    /// Sets a key in the config.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `key: &str` - The key
    /// * `value: &str` - The value
    ///
    /// ### Examples
    ///
    /// ```
    /// let mut config = config::Config::empty();
    /// config.set("key", "value");
    /// ```
    pub fn set(&mut self, key: &str, value: &str) {
        let json = self.translate(key);

        match json {
            Ok(json) => {
                *json = serde_json::Value::String(value.to_string());
                self.state = State::Unsaved;
            }
            Err(err) => {
                debug!("Error setting config key {}: {}", key, err);
            }
        }
    }

    /// ## get_or(&mut self, key: &str, default: impl Fn() -> String) -> Result<String, String>
    ///
    /// Gets a key from the config, or sets it to a default value if it doesn't exist.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    ///
    /// ### Returns
    ///
    /// * `key: &str` - The key
    /// * `default: impl Fn() -> String` - The default value
    ///
    /// ### Returns
    ///
    /// * `Result<String, String>` - The result of the command
    ///
    /// ### Examples
    ///
    /// ```
    /// let mut config = config::Config::empty();
    /// let value = config.get_or("key", || "default".to_string());
    /// ```
    pub fn get_or(&mut self, key: &str, default: impl Fn() -> String) -> Result<String, String> {
        let value = self.translate(key);
        match value {
            Ok(value) => match value.as_str() {
                Some(value) => Ok(value.to_string()),
                None => {
                    let default = default();
                    self.set(key, &default);
                    debug!("{} is not set. Setting to default value {}.", key, default);
                    Ok(default)
                }
            },
            Err(err) => {
                let default = default();
                self.set(key, &default);
                debug!("{} is not set. Setting to default value {}.", key, default);
                Ok(default)
            }
        }
    }

    /// ## empty() -> Self
    ///
    /// Creates an empty config.
    ///
    /// ### Returns
    ///
    /// * `Self` - The config
    pub fn empty() -> Self {
        Self {
            state: State::Unsaved,
            json: serde_json::Value::Null,
        }
    }

    /// ## load_from_file(path: &str) -> Result<Self, String>
    ///
    /// Loads a config from a file.
    ///
    /// ### Arguments
    ///
    /// * `path: &str` - The path to the file
    ///
    /// ### Returns
    ///
    /// * `Result<Self, String>` - The result of the command
    ///
    /// ### Examples
    ///
    /// ```
    /// let mut config = config::Config::load_from_file("config.json");
    /// ```
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        // Create file if it doesn't exist
        if !std::path::Path::new(path).exists() {
            let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
            serde_json::to_writer_pretty(file, &serde_json::Value::Null)
                .map_err(|e| e.to_string())?;
        }

        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let json = serde_json::from_reader(file).map_err(|e| e.to_string())?;
        Ok(Self {
            state: State::Saved,
            json,
        })
    }

    /// ## save_to_file(&mut self, path: &str) -> Result<(), String>
    ///
    /// Saves a config to a file.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `path: &str` - The path to the file
    ///
    /// ### Returns
    ///
    /// * `Ok(())` - The result of the command
    /// * `Err(String)` - The result of the command
    ///
    /// ### Examples
    ///
    /// ```
    /// let mut config = config::Config::empty();
    /// config.save_to_file("config.json");
    /// ```
    pub fn save_to_file(&mut self, path: &str) -> Result<(), String> {
        let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
        serde_json::to_writer_pretty(file, &self.json).map_err(|e| e.to_string())?;
        self.state = State::Saved;
        Ok(())
    }
}
