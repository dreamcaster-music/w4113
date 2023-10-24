/*


Ronin
I started writing this and then realized I am basically recreating JSON but worse
So I'm going to stop here and just use JSON instead
I'll leave this here in case I ever want to come back to it

There's probably a library to do JSON in Rust

Wesley out


*/






use std::fmt::{Display, Formatter};

// ConfigState
// Used to track whether the config has been saved or not
enum ConfigState {
	Saved,
	Unsaved,
}

// Config
// Used to store the configuration of the application and is designed to be directly linked to a file
struct Config {
	path: String,
	state: ConfigState,
	root: ConfigObject,
}

// ConfigObject
// Basically a node in a tree structure that represents the configuration
enum ConfigObject {
	Branch(ConfigBranch),
	Leaf(ConfigLeaf),
}

// ConfigBranch
// A branch in the configuration tree. A branch is just a ConfigObject where instead of a value, it has a list of children
struct ConfigBranch {
	name: String,
	id: String,
	children: Vec<ConfigObject>,
}

// ConfigLeaf
// A leaf in the configuration tree. This is where values are stored
struct ConfigLeaf {
	name: String,
	id: String,
	value: String,
}

impl Config {
	fn parse(data: String) -> Result<ConfigBranch, String> {
		// Create the root node
		let mut node = ConfigBranch {
			name: "root".to_string(),
			id: "root".to_string(),
			children: Vec::new(),
		};

		// Set some constants for the state machine
		let UNKNOWN = 0;
		let OBJECT_NAME = 1;
		let OBJECT_VALUE = 2;

		let mut state = OBJECT_NAME;
		let mut current_string = String::new();

		let mut i = 0;
		for c in data.chars() {
			if state == OBJECT_NAME {
				if c.is_whitespace() {
					// Object names can't have whitespace in them
					if current_string.len() > 0 {
						return Err("Unexpected whitespace".to_string());
					} else {
						continue;
					}
				} else if c == '{' {
					return Err("Unexpected {".to_string());
				} else if c == '}' {
					return Err("Unexpected }".to_string());
				} else if c == '.' {
					return Err("Unexpected .".to_string());
				} else if c == ';' {
					return Err("Unexpected ;".to_string());
				} else if c == ':' {
					state = OBJECT_VALUE;
					node.children.push(ConfigObject::Leaf(ConfigLeaf {
						name: current_string.clone(),
						id: node.id.clone() + "." + &current_string,
						value: "".to_string(),
					}));
					current_string.clear();
				} else {
					current_string.push(c);
				}
			} else if state == OBJECT_VALUE {
				
			}
			i += 1;
		}

		Err("Not implemented".to_string())
	}

	fn open(path: String) -> Result<Config, String> {
		// Read the input file
		let file = std::fs::read_to_string(&path);

		// Create the config object from the file
		let config = match file {
			Ok(file) => {

				// If the file exists and was read successfully, generate the config object from it
				let root = Config::parse(file);

				match root {
					Ok(root) => Config {
						path,
						state: ConfigState::Saved,
						root: ConfigObject::Branch(root)
					},
					Err(e) => return Err(e),
				}
			},
			Err(_) => {

				// If the file doesn't exist or couldn't be read, create a new empty config object
				let root = ConfigObject::Branch(ConfigBranch {
					name: "root".to_string(),
					id: "root".to_string(),
					children: Vec::new(),
				});
				Config {
					path,
					state: ConfigState::Unsaved,
					root
				}
			},
		};

		// Return the config object
		Ok(config)
	}
}

// TODO: Implement Display for Config

impl Display for Config {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\n", self.path)
	}
}