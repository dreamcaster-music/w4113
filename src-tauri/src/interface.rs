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
	#[cfg(target_os = "macos")]
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

// MacOS/Linux
#[cfg(target_os = "macos")]
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

// Windows
#[cfg(target_os = "windows")]
pub fn get_interfaces() -> Vec<Interface> {
	let mut interfaces: Vec<Interface> = Vec::new();
	// use winapi to get the device list
	let mut device_info_set = unsafe { winapi::um::setupapi::SetupDiGetClassDevsA(
		&winapi::um::winnt::GUID_DEVINTERFACE_HID,
		std::ptr::null(),
		std::ptr::null_mut(),
		winapi::um::setupapi::DIGCF_PRESENT | winapi::um::setupapi::DIGCF_DEVICEINTERFACE,
	) };
	return interfaces;
}