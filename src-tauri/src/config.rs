//! Contains all the code needed to load and save w4113's JSON config files
//! 
//! The config files are stored in the `target/config` directory. This directory is created if it does not exist.

use log::debug;

/// ## CONFIG_ROOT
/// 
/// The root directory for any config files
pub static CONFIG_ROOT: &str = "target/config";

/// ## ConfigState
/// 
/// Stores whether a config has been saved or not
/// 
/// ### Variants
/// 
/// * `Saved` - The config has been saved
/// * `Unsaved` - The config has not been saved
pub enum ConfigState {
    Saved,
    Unsaved,
}

/// ## Config
/// 
/// A config struct with a reference to a .json file
/// 
/// ### Fields
/// 
/// * `state` - The state of the config
/// * `config_path` - The path to the config
/// * `config` - The config
/// 
/// ### Methods
/// 
/// * `state() -> &ConfigState` - Returns a reference to the config's state
/// * `config_path() -> &str` - Returns a reference to the config's path
/// * `json() -> &serde_json::Value` - Returns a reference to the config's JSON
/// * `json_mut() -> &mut serde_json::Value` - Returns a mutable reference to the config's JSON
/// * `load(path: &str) -> Result<Config, String>` - Loads a config from a path
/// * `save(config: &Config) -> Result<(), String>` - Saves a config to a path
pub struct Config {
    state: ConfigState,
    config_path: String,
    config: serde_json::Value,
}

impl Config {

	/// ## state() -> &ConfigState
	/// 
	/// Returns a reference to the config's state
	/// 
	/// ### Returns
	/// 
	/// * `&ConfigState` - The config's state
	pub fn state(&self) -> &ConfigState {
		&self.state
	}

	/// ## config_path() -> &str
	/// 
	/// Returns a reference to the config's path
	/// 
	/// ### Returns
	/// 
	/// * `&str` - The config's path
	pub fn config_path(&self) -> &str {
		&self.config_path
	}

	/// ## json() -> &serde_json::Value
	/// 
	/// Returns a reference to the config's JSON
	/// 
	/// ### Returns
	/// 
	/// * `&serde_json::Value` - The config's JSON
	pub fn json(&self) -> &serde_json::Value {
		&self.config
	}

	/// ## json_mut() -> &mut serde_json::Value
	/// 
	/// Returns a mutable reference to the config's JSON
	/// 
	/// ### Returns
	/// 
	/// * `&mut serde_json::Value` - The config's JSON
	pub fn json_mut(&mut self) -> &mut serde_json::Value {
		self.state = ConfigState::Unsaved;
		&mut self.config
	}

	/// ## load(path: &str) -> Result<Config, String>
	/// 
	/// Loads a config from a path
	/// 
	/// ### Arguments
	/// 
	/// * `path: &str` - The path to the config
	/// 
	/// ### Returns
	/// 
	/// * `Ok(Config)` - The config
	/// * `Err(String)` - The error message
	/// 
	/// ### Examples
	/// 
	/// ```
	/// let config = Config::load("config.json").unwrap();
	/// ```
    pub fn load(path: &str) -> Result<Config, String> {
        debug!("Loading config from {}", path);

        if !std::path::Path::new(&(CONFIG_ROOT.to_owned() + path)).exists() {
            // Config does not exist, create the config
            debug!("Config does not exist, creating");
            let config = serde_json::json!({});
            let config_str = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
            std::fs::write(CONFIG_ROOT.to_owned() + path, config_str).map_err(|e| e.to_string())?;
            return Ok(Config {
                state: ConfigState::Unsaved,
                config_path: path.to_owned(),
                config,
            });
        } else {
            // Config exists, load existing config
            debug!("Config exists, loading");
            let config_str = std::fs::read_to_string(CONFIG_ROOT.to_owned() + path)
                .map_err(|e| e.to_string())?;
            let config = serde_json::from_str(&config_str).map_err(|e| e.to_string())?;
            return Ok(Config {
                state: ConfigState::Saved,
                config_path: path.to_owned(),
                config,
            });
        }
    }

	/// ## save(config: &Config) -> Result<(), String>
	/// 
	/// Saves a config to a path
	/// 
	/// ### Arguments
	/// 
	/// * `config: &Config` - The config to save
	/// 
	/// ### Returns
	/// 
	/// * `Ok(())` - The config was saved successfully
	/// * `Err(String)` - The error message
	/// 
	/// ### Examples
	/// 
	/// ```
	/// let config = Config::load("config.json").unwrap();
	/// Config::save(&config).unwrap();
	/// ```
    pub fn save(config: &Config) -> Result<(), String> {
        debug!("Saving config to {}", config.config_path);
        let config_str = serde_json::to_string_pretty(&config.config).map_err(|e| e.to_string())?;
        std::fs::write(CONFIG_ROOT.to_owned() + &config.config_path, config_str)
            .map_err(|e| e.to_string())?;
        debug!("Saved config");
        Ok(())
    }
}
