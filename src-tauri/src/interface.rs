// Listen to keyboard events using /dev/input

use std::{sync::RwLock, fmt::{Formatter, Display}};

use hidapi::HidApi;
use log::{debug, error};

use lazy_static::lazy_static;

lazy_static! {
    static ref API: HidApi = HidApi::new().unwrap();
    static ref KEYS: RwLock<Vec<u8>> = RwLock::new(vec![0u8; 8]);
}

pub struct Interface {
	id: u32,
	manufacturer: String,
	product: String,
	serial: String,
	handles: Vec<hidapi::HidDevice>,
}

impl Display for Interface {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"({}) {}: {}",
			self.id, self.manufacturer, self.product
		)
	}
}

impl PartialEq for Interface {
	fn eq(&self, other: &Self) -> bool {
		self.manufacturer == other.manufacturer
			&& self.product == other.product
			&& self.serial == other.serial
			&& self.id == other.id
	}
}

impl Interface {
	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn manufacturer(&self) -> &str {
		&self.manufacturer
	}

	pub fn product(&self) -> &str {
		&self.product
	}

	pub fn serial(&self) -> &str {
		&self.serial
	}
}

fn hash(string: String) -> u32 {
	let mut hash: u32 = 0;
	for c in string.chars() {
		hash = (hash as u32).wrapping_mul(31).wrapping_add(c as u32);
	}
	return hash;
}

pub fn get_interfaces() -> Vec<Interface> {
	let mut interfaces: Vec<Interface> = Vec::new();
	for device in API.device_list() {
		let manufacturer = device.manufacturer_string().unwrap_or("Unknown");
		let product = device.product_string().unwrap_or("Unknown");
		let serial = device.serial_number().unwrap_or("Unknown");
		let vendor_id = device.vendor_id();
		let product_id = device.product_id();

		let combo = manufacturer.to_owned() + &product + &serial;
		let id = hash(combo);

		let mut new_interface = Interface {
			id: id,
			manufacturer: manufacturer.to_owned(),
			product: product.to_owned(),
			serial: serial.to_owned(),
			handles: Vec::new(),
		};

		// check if the interface already exists
		let mut exists = false;
		for interface in interfaces.iter_mut() {
			if interface == &new_interface {
				interface.handles.push(device.open_device(&API).unwrap());
				exists = true;
				break;
			}
		}

		if !exists {
			new_interface.handles.push(device.open_device(&API).unwrap());
			interfaces.push(new_interface);	
		}
	}
	return interfaces;
}

// pub fn hid_list() -> Result<Vec<String>, String> {
//     let mut devices = Vec::new();
//     for device in API.device_list() {
//         //let device_handle = device.open_device(&api).unwrap();
//         let manufacturer = device.manufacturer_string().unwrap_or("Unknown");
//         let product = device.product_string().unwrap_or("Unknown");
//         let serial = device.serial_number().unwrap_or("Unknown");
//         let path = device.path().to_str().unwrap_or("Unknown");

//         let vendor_id = device.vendor_id();
//         let product_id = device.product_id();

//         let valid = vendor_id == 1452 || product_id == 641 || product.contains("USB");

//         if valid {
//             let thread = std::thread::spawn(move || {
//                 debug!(
//                     "Found Apple keyboard: {} : {} : {} : {}",
//                     manufacturer, product, serial, path
//                 );
//                 loop {
//                     let handle = device.open_device(&API).unwrap();
//                     let mut buf = [0u8; 8];
//                     let _ = handle.read(&mut buf);
//                     let mut compare = false;
//                     if buf[0 as usize] == 1 {
//                         debug!("Found key: {:?}", buf);
//                         let mut keys = match KEYS.write() {
//                             Ok(keys) => keys,
//                             Err(e) => {
//                                 error!("Error getting keys: {}", e);
//                                 continue;
//                             }
//                         };

//                         // compare the buf to the current keys; if they are the same, the thread is redundant and needs to be killed
//                         let buf_vec = buf.to_vec();

//                         compare = true;
//                         for i in 0..8 {
//                             if buf_vec[i] != keys[i] {
//                                 compare = false;
//                                 break;
//                             }
//                         }

//                         *keys = buf_vec;
//                     }

//                     if compare {
//                         break;
//                     }
//                 }
//                 debug!("Ending thread.");
//             });
//         }

//         let device = format!("{} : {} : {} : {}", manufacturer, product, serial, path);
//         devices.push(format!("{:?}", device));
//     }
//     return Ok(devices);
// }
