pub enum State {
	Unsaved,
	Saved,
}

pub struct Config {
	state: State,
	json: serde_json::Value,
}

impl Config {
	pub fn state(&self) -> &State {
		&self.state
	}

	pub fn set(&mut self, key: &str, value: &str) {
		self.json[key] = serde_json::Value::String(value.to_string());
		self.state = State::Unsaved;
	}

	// default: A callback that returns String
	pub fn get_or(&mut self, key: &str, default: impl Fn() -> String) -> Result<String, String> {
		let value = self.json.get(key);
		match value {
			Some(value) => {
				match value.as_str() {
					Some(value) => Ok(value.to_string()),
					None => Err(format!("{} is not a string", key)),
				}
			}
			None => {
				let default = default();
				self.set(key, &default);
				Ok(default)
			}
		}
	}

	pub fn empty() -> Self {
		Self {
			state: State::Unsaved,
			json: serde_json::Value::Null,
		}
	}

	pub fn load_from_file(path: &str) -> Result<Self, String> {
		// Create file if it doesn't exist
		if !std::path::Path::new(path).exists() {
			let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
			serde_json::to_writer_pretty(file, &serde_json::Value::Null).map_err(|e| e.to_string())?;
		}

		let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
		let json = serde_json::from_reader(file).map_err(|e| e.to_string())?;
		Ok(Self {
			state: State::Saved,
			json,
		})
	}

	pub fn save_to_file(&mut self, path: &str) -> Result<(), String> {
		let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
		serde_json::to_writer_pretty(file, &self.json).map_err(|e| e.to_string())?;
		self.state = State::Saved;
		Ok(())
	}
}