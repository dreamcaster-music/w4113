// Listen to keyboard events using /dev/input

use hidapi::HidApi;
use log::{error, debug};

pub fn hid_list() -> Result<Vec<String>, String> {
	match HidApi::new() {
		Ok(api) => {
			let mut devices = Vec::new();
			for device in api.device_list() {

				//let device_handle = device.open_device(&api).unwrap();
				let manufacturer = device.manufacturer_string().unwrap_or("Unknown");
				let product = device.product_string().unwrap_or("Unknown");
				// debug!("Manufacturer: {}", manufacturer);
				// debug!("Product: {}", product);
				// debug!("Other: {:?}", device);
				// loop {
				// 	debug!("Reading");
				// 	let mut buf = [0u8; 8];
				// 	device_handle.read(&mut buf).unwrap();
				// 	debug!("{:?}", buf);
				// }

				let device = format!("{} : {}", manufacturer, product);
				devices.push(format!("{:?}", device));
			}
			return Ok(devices);
		},
		Err(e) => {
			error!("Error opening HID API. {:?}", e);
			return Err(format!("{:?}", e));
		},
	}
}