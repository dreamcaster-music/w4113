//! ## config.rs
//!
//! This module is used for anything related to configuration and in the filesystem.

use log::debug;

/// ## State
///
/// Represents the state of a config.
///
/// ### Variants
///
/// * `Unsaved` - The config has been modified since it was last saved
/// * `Saved` - The config has not been modified since it was last saved
#[derive(Clone, PartialEq)]
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
/// * `on_update: Option<Box<dyn Fn(&mut Config)>>` - The function to call when the config is updated
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
pub struct Config {
    on_update: Option<Box<dyn Fn(&mut Config) + Send>>,
    updating: bool,
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
            let _temp = match value[key] {
                _ => {
                    value = &mut value[key];
                }
            };
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
    pub fn set_str(&mut self, key: &str, value: &str) {
        let json = self.translate(key);

        match json {
            Ok(json) => {
                *json = serde_json::Value::String(value.to_string());
                self.state = State::Unsaved;
                self.run_update();
            }
            Err(err) => {
                debug!("Error setting config key {}: {}", key, err);
            }
        }
    }

    /// ## set_num(&mut self, key: &str, value: i64)
    ///
    /// Sets a key in the config.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `key: &str` - The key
    /// * `value: i64` - The value
    pub fn set_num(&mut self, key: &str, value: i64) {
        let json = self.translate(key);

        match json {
            Ok(json) => {
                *json =
                    serde_json::Value::Number(serde_json::Number::from_f64(value as f64).unwrap());
                self.state = State::Unsaved;
                self.run_update();
            }
            Err(err) => {
                debug!("Error setting config key {}: {}", key, err);
            }
        }
    }

    /// ## set_bool(&mut self, key: &str, value: bool)
    ///
    /// Sets a key in the config.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `key: &str` - The key
    /// * `value: bool` - The value
    pub fn set_bool(&mut self, key: &str, value: bool) {
        let json = self.translate(key);

        match json {
            Ok(json) => {
                *json = serde_json::Value::Bool(value);
                self.state = State::Unsaved;
                self.run_update();
            }
            Err(err) => {
                debug!("Error setting config key {}: {}", key, err);
            }
        }
    }

    /// ## set_value(&mut self, key: &str, value: serde_json::Value)
    ///
    /// Sets a key in the config.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `key: &str` - The key
    /// * `value: serde_json::Value` - The value
    pub fn set_value(&mut self, key: &str, value: serde_json::Value) {
        let json = self.translate(key);

        match json {
            Ok(json) => {
                *json = value;
                self.state = State::Unsaved;
                self.run_update();
            }
            Err(err) => {
                debug!("Error setting config key {}: {}", key, err);
            }
        }
    }

    /// ## set_array(&mut self, key: &str, value: Vec<serde_json::Value>)
    ///
    /// Sets a key in the config.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `key: &str` - The key
    /// * `value: Vec<serde_json::Value>` - The value
    pub fn set_array(&mut self, key: &str, value: Vec<serde_json::Value>) {
        let json = self.translate(key);

        match json {
            Ok(json) => {
                *json = serde_json::Value::Array(value);
                self.state = State::Unsaved;
                self.run_update();
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
	#[allow(dead_code)]
    pub fn get_str_or(
        &mut self,
        key: &str,
        default: impl Fn() -> String,
    ) -> Result<String, String> {
        let value = self.translate(key);
        match value {
            Ok(value) => match value.as_str() {
                Some(value) => Ok(value.to_string()),
                None => {
                    let default = default();
                    self.set_str(key, &default);
                    debug!("{} is not set. Setting to default value {}.", key, default);
                    Ok(default)
                }
            },
            Err(_err) => {
                let default = default();
                self.set_str(key, &default);
                debug!("{} is not set. Setting to default value {}.", key, default);
                Ok(default)
            }
        }
    }

    /// ## get_num_or(&mut self, key: &str, default: impl Fn() -> i64) -> Result<i64, String>
    ///
    /// Gets a key from the config, or sets it to a default value if it doesn't exist.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `key: &str` - The key
    /// * `default: impl Fn() -> i64` - The default value
    ///
    /// ### Returns
    ///
    /// * `Result<i64, String>` - The result of the command
	#[allow(dead_code)]
    pub fn get_num_or(&mut self, key: &str, default: impl Fn() -> i64) -> Result<i64, String> {
        let value = self.translate(key);
        match value {
            Ok(value) => match value.as_i64() {
                Some(value) => Ok(value),
                None => {
                    let default = default();
                    self.set_num(key, default);
                    debug!("{} is not set. Setting to default value {}.", key, default);
                    Ok(default)
                }
            },
            Err(_err) => {
                let default = default();
                self.set_num(key, default);
                debug!("{} is not set. Setting to default value {}.", key, default);
                Ok(default)
            }
        }
    }

    /// ## get_bool_or(&mut self, key: &str, default: impl Fn() -> bool) -> Result<bool, String>
    ///
    /// Gets a key from the config, or sets it to a default value if it doesn't exist.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `key: &str` - The key
    /// * `default: impl Fn() -> bool` - The default value
    ///
    /// ### Returns
    ///
    /// * `Result<bool, String>` - The result of the command
	#[allow(dead_code)]
    fn get_bool_or(&mut self, key: &str, default: impl Fn() -> bool) -> Result<bool, String> {
        let value = self.translate(key);
        match value {
            Ok(value) => match value.as_bool() {
                Some(value) => Ok(value),
                None => {
                    let default = default();
                    self.set_bool(key, default);
                    debug!("{} is not set. Setting to default value {}.", key, default);
                    Ok(default)
                }
            },
            Err(_err) => {
                let default = default();
                self.set_bool(key, default);
                debug!("{} is not set. Setting to default value {}.", key, default);
                Ok(default)
            }
        }
    }

    /// ## get_value_or(&mut self, key: &str, default: impl Fn() -> serde_json::Value) -> Result<serde_json::Value, String>
    ///
    /// Gets a key from the config, or sets it to a default value if it doesn't exist.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `key: &str` - The key
    /// * `default: impl Fn() -> serde_json::Value` - The default value
    ///
    /// ### Returns
    ///
    /// * `Result<serde_json::Value, String>` - The result of the command
	#[allow(dead_code)]
    pub fn get_value_or(
        &mut self,
        key: &str,
        default: impl Fn() -> serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let value = self.translate(key);
        match value {
            Ok(value) => match value {
                _ => Ok(value.clone()),
            },
            Err(_err) => {
                let default = default();
                self.set_value(key, default.clone());
                debug!("{} is not set. Setting to default value.", key);
                Ok(default)
            }
        }
    }

    /// ## get_array_or(&mut self, key: &str, default: impl Fn() -> Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>, String>
    ///
    /// Gets a key from the config, or sets it to a default value if it doesn't exist.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `key: &str` - The key
    /// * `default: impl Fn() -> Vec<serde_json::Value>` - The default value
    ///
    /// ### Returns
    ///
    /// * `Result<Vec<serde_json::Value>, String>` - The result of the command
	#[allow(dead_code)]
    pub fn get_array_or(
        &mut self,
        key: &str,
        default: impl Fn() -> Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, String> {
        let value = self.translate(key);
        match value {
            Ok(value) => match value.as_array() {
                Some(value) => Ok(value.to_vec()),
                None => {
                    let default = default();
                    self.set_array(key, default.clone());
                    debug!("{} is not set. Setting to default value.", key);
                    Ok(default.to_vec())
                }
            },
            Err(_err) => {
                let default = default();
                self.set_array(key, default.clone());
                debug!("{} is not set. Setting to default value.", key);
                Ok(default.to_vec())
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
            on_update: None,
            updating: false,
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
            on_update: None,
            updating: false,
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
        self.run_update();
        Ok(())
    }

    /// ## on_update(&mut self, f: impl Fn(&mut Config) + 'static)
    ///
    /// Sets the function to call when the config is updated.
    /// Note that on_update is called once after being set.
    /// on_update is called when the config is saved or changed.
    /// Note that on_update is always called AFTER a change.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    /// * `f: impl Fn(&mut Config) + 'static` - The function to call
    ///
    /// ### Examples
    ///
    /// ```
    /// let mut config = config::Config::empty();
    /// config.on_update(|config| {
    /// 	println!("Config updated!");
    /// });
    pub fn on_update(&mut self, f: impl Fn(&mut Config) + Send + 'static) {
        if !self.updating {
            self.on_update = Some(Box::new(f));
            self.run_update();
        }
    }

    /// ## run_update(&mut self)
    ///
    /// Private function which runs the on_update function.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config
    fn run_update(&mut self) {
        if !self.updating {
            self.updating = true;
            let mut partial_clone = self.partial_clone();
            match &self.on_update {
                Some(f) => f(&mut partial_clone),
                None => (),
            }
            self.state = partial_clone.state;
            self.json = partial_clone.json;
            self.updating = false;
        } else {
        }
    }

    /// ## partial_clone(&self) -> Self
    ///
    /// Clones the config, but without the on_update function.
    ///
    /// ### Arguments
    ///
    /// * `&self` - The config
    ///
    /// ### Returns
    ///
    /// * `Self` - The config
    pub fn partial_clone(&self) -> Self {
        Self {
            on_update: None,
            updating: self.updating,
            state: self.state.clone(),
            json: self.json.clone(),
        }
    }
}
