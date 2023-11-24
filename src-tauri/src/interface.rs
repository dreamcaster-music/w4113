// Listen to keyboard events using /dev/input

use std::sync::RwLock;

use hidapi::HidApi;
use log::{debug, error};

use lazy_static::lazy_static;

lazy_static! {
    static ref API: HidApi = HidApi::new().unwrap();
    static ref KEYS: RwLock<Vec<u8>> = RwLock::new(vec![0u8; 8]);
}

pub fn hid_list() -> Result<Vec<String>, String> {
    let mut devices = Vec::new();
    for device in API.device_list() {
        //let device_handle = device.open_device(&api).unwrap();
        let manufacturer = device.manufacturer_string().unwrap_or("Unknown");
        let product = device.product_string().unwrap_or("Unknown");
        let serial = device.serial_number().unwrap_or("Unknown");
        let path = device.path().to_str().unwrap_or("Unknown");

        let vendor_id = device.vendor_id();
        let product_id = device.product_id();

        let valid = vendor_id == 1452 || product_id == 641 || product.contains("USB");

        if valid {
            let thread = std::thread::spawn(move || {
                debug!(
                    "Found Apple keyboard: {} : {} : {} : {}",
                    manufacturer, product, serial, path
                );
                loop {
                    let handle = device.open_device(&API).unwrap();
                    let mut buf = [0u8; 8];
                    let _ = handle.read(&mut buf);
                    let mut compare = false;
                    if buf[0 as usize] == 1 {
                        debug!("Found key: {:?}", buf);
                        let mut keys = match KEYS.write() {
                            Ok(keys) => keys,
                            Err(e) => {
                                error!("Error getting keys: {}", e);
                                continue;
                            }
                        };

                        // compare the buf to the current keys; if they are the same, the thread is redundant and needs to be killed
                        let buf_vec = buf.to_vec();

                        compare = true;
                        for i in 0..8 {
                            if buf_vec[i] != keys[i] {
                                compare = false;
                                break;
                            }
                        }

                        *keys = buf_vec;
                    }

                    if compare {
                        break;
                    }
                }
                debug!("Ending thread.");
            });
        }

        let device = format!("{} : {} : {} : {}", manufacturer, product, serial, path);
        devices.push(format!("{:?}", device));
    }
    return Ok(devices);
}
