//! ## config.rs
//!
//! This module is used for anything related to configuration and in the filesystem.

use log::error;

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
            json: serde_json::Value::Null,
        }
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

    /// Saves the config to a different path.
    ///
    /// ### Arguments
    ///
    /// * `&mut self` - The config.
    /// * `path: &str` - The path to save the config to.
    ///
    /// ### Returns
    ///
    /// * `Result<(), String>` - Whether or not the config was saved.
    pub fn save_to(&mut self, path: &str) -> Result<(), String> {
        let file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
        serde_json::to_writer_pretty(file, &self.json).map_err(|e| e.to_string())?;
        self.saved = true;
        self.path = path.to_string();
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
