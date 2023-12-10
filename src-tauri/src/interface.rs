// Listen to keyboard events using /dev/input

use std::{
    fmt::{Display, Formatter},
    sync::{Arc, Mutex, RwLock, RwLockReadGuard},
};

use hidapi::{HidApi, HidDevice};
use log::{debug, error, trace};

use lazy_static::lazy_static;

lazy_static! {
    static ref API: HidApi = HidApi::new().unwrap();
}

#[allow(dead_code)]
#[derive(ts_rs::TS)]
#[ts(export, export_to = "../src/bindings/Key.ts")]
pub enum Key {
    Unknown = 0,
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,
    Num1 = 30,
    Num2 = 31,
    Num3 = 32,
    Num4 = 33,
    Num5 = 34,
    Num6 = 35,
    Num7 = 36,
    Num8 = 37,
    Num9 = 38,
    Num0 = 39,
    Enter = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Equals = 46,
    LeftBracket = 47,
    RightBracket = 48,
    Backslash = 49,
    NonUsHash = 50,
    Semicolon = 51,
    Apostrophe = 52,
    Grave = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::A => write!(f, "A"),
            Key::B => write!(f, "B"),
            Key::C => write!(f, "C"),
            Key::D => write!(f, "D"),
            Key::E => write!(f, "E"),
            Key::F => write!(f, "F"),
            Key::G => write!(f, "G"),
            Key::H => write!(f, "H"),
            Key::I => write!(f, "I"),
            Key::J => write!(f, "J"),
            Key::K => write!(f, "K"),
            Key::L => write!(f, "L"),
            Key::M => write!(f, "M"),
            Key::N => write!(f, "N"),
            Key::O => write!(f, "O"),
            Key::P => write!(f, "P"),
            Key::Q => write!(f, "Q"),
            Key::R => write!(f, "R"),
            Key::S => write!(f, "S"),
            Key::T => write!(f, "T"),
            Key::U => write!(f, "U"),
            Key::V => write!(f, "V"),
            Key::W => write!(f, "W"),
            Key::X => write!(f, "X"),
            Key::Y => write!(f, "Y"),
            Key::Z => write!(f, "Z"),
            Key::Num1 => write!(f, "Num1"),
            Key::Num2 => write!(f, "Num2"),
            Key::Num3 => write!(f, "Num3"),
            Key::Num4 => write!(f, "Num4"),
            Key::Num5 => write!(f, "Num5"),
            Key::Num6 => write!(f, "Num6"),
            Key::Num7 => write!(f, "Num7"),
            Key::Num8 => write!(f, "Num8"),
            Key::Num9 => write!(f, "Num9"),
            Key::Num0 => write!(f, "Num0"),
            Key::Enter => write!(f, "Enter"),
            Key::Escape => write!(f, "Escape"),
            Key::Backspace => write!(f, "Backspace"),
            Key::Tab => write!(f, "Tab"),
            Key::Space => write!(f, "Space"),
            Key::Minus => write!(f, "Minus"),
            Key::Equals => write!(f, "Equals"),
            Key::LeftBracket => write!(f, "LeftBracket"),
            Key::RightBracket => write!(f, "RightBracket"),
            Key::Backslash => write!(f, "Backslash"),
            Key::NonUsHash => write!(f, "NonUsHash"),
            Key::Semicolon => write!(f, "Semicolon"),
            Key::Apostrophe => write!(f, "Apostrophe"),
            Key::Grave => write!(f, "Grave"),
            Key::Comma => write!(f, "Comma"),
            Key::Period => write!(f, "Period"),
            Key::Slash => write!(f, "Slash"),
            Key::CapsLock => write!(f, "CapsLock"),
            Key::F1 => write!(f, "F1"),
            Key::F2 => write!(f, "F2"),
            Key::F3 => write!(f, "F3"),
            Key::F4 => write!(f, "F4"),
            Key::F5 => write!(f, "F5"),
            Key::F6 => write!(f, "F6"),
            Key::F7 => write!(f, "F7"),
            Key::F8 => write!(f, "F8"),
            Key::F9 => write!(f, "F9"),
            Key::F10 => write!(f, "F10"),
            Key::F11 => write!(f, "F11"),
            Key::F12 => write!(f, "F12"),
            Key::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Key {
    pub fn from(code: u8) -> Key {
        match code {
            4 => Key::A,
            5 => Key::B,
            6 => Key::C,
            7 => Key::D,
            8 => Key::E,
            9 => Key::F,
            10 => Key::G,
            11 => Key::H,
            12 => Key::I,
            13 => Key::J,
            14 => Key::K,
            15 => Key::L,
            16 => Key::M,
            17 => Key::N,
            18 => Key::O,
            19 => Key::P,
            20 => Key::Q,
            21 => Key::R,
            22 => Key::S,
            23 => Key::T,
            24 => Key::U,
            25 => Key::V,
            26 => Key::W,
            27 => Key::X,
            28 => Key::Y,
            29 => Key::Z,
            30 => Key::Num1,
            31 => Key::Num2,
            32 => Key::Num3,
            33 => Key::Num4,
            34 => Key::Num5,
            35 => Key::Num6,
            36 => Key::Num7,
            37 => Key::Num8,
            38 => Key::Num9,
            39 => Key::Num0,
            40 => Key::Enter,
            41 => Key::Escape,
            42 => Key::Backspace,
            43 => Key::Tab,
            44 => Key::Space,
            45 => Key::Minus,
            46 => Key::Equals,
            47 => Key::LeftBracket,
            48 => Key::RightBracket,
            49 => Key::Backslash,
            50 => Key::NonUsHash,
            51 => Key::Semicolon,
            52 => Key::Apostrophe,
            53 => Key::Grave,
            54 => Key::Comma,
            55 => Key::Period,
            56 => Key::Slash,
            57 => Key::CapsLock,
            58 => Key::F1,
            59 => Key::F2,
            60 => Key::F3,
            61 => Key::F4,
            62 => Key::F5,
            63 => Key::F6,
            64 => Key::F7,
            65 => Key::F8,
            66 => Key::F9,
            67 => Key::F10,
            68 => Key::F11,
            69 => Key::F12,
            _ => Key::Unknown,
        }
    }
}

#[derive(ts_rs::TS)]
#[ts(export, export_to = "../src/bindings/Mod.ts")]
enum Mod {
    LeftControl = 1,

    LeftShift = 2,
    RightShift = 32,

    // Alt is equivalent to Option on Mac
    LeftAlt = 4,
    RightAlt = 64,

    // Windows key on Windows, Command on Mac
    LeftSuper = 8,
    RightSuper = 128,
}

impl Display for Mod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mod::LeftControl => write!(f, "LeftControl"),
            Mod::LeftShift => write!(f, "LeftShift"),
            Mod::RightShift => write!(f, "RightShift"),
            Mod::LeftAlt => write!(f, "LeftAlt"),
            Mod::RightAlt => write!(f, "RightAlt"),
            Mod::LeftSuper => write!(f, "LeftSuper"),
            Mod::RightSuper => write!(f, "RightSuper"),
        }
    }
}

pub struct Interface {
    id: u32,
    manufacturer: String,
    product: String,
    serial: String,
    keydown_callback: Arc<RwLock<Option<Box<dyn Fn(Key) + 'static + Sync + Send>>>>,
    keyup_callback: Arc<RwLock<Option<Box<dyn Fn(Key) + 'static + Sync + Send>>>>,
    keys: Arc<RwLock<Vec<u8>>>,
    handles: RwLock<Vec<HidDevice>>,
}

impl Display for Interface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {}: {}", self.id, self.manufacturer, self.product)
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

impl<'a> Interface {
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

    pub fn keydown(&mut self, callback: Box<dyn Fn(Key) + 'static + Sync + Send>) {
        let mut callback_ref = match self.keydown_callback.write() {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get keydown callback: {}", e);
                return;
            }
        };

        *callback_ref = Some(callback);
    }

    pub fn keyup(&mut self, callback: Box<dyn Fn(Key) + 'static + Sync + Send>) {
        let mut callback_ref = match self.keyup_callback.write() {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get keyup callback: {}", e);
                return;
            }
        };

        *callback_ref = Some(callback);
    }

    pub fn thread(&mut self) {
        let mut handles = {
            match self.handles.write() {
                Ok(h) => h,
                Err(e) => {
                    error!("Failed to get handles: {}", e);
                    return;
                }
            }
        };

        let len = handles.len();
        for _i in 0..len {
            let keys_reference = self.keys.clone();
            let keydown_callback_reference = self.keydown_callback.clone();
            let keyup_callback_reference = self.keyup_callback.clone();
            let handle = handles.remove(0);
            let _thread = std::thread::spawn(move || {
                let mut buf = [0u8; 8];
                loop {
                    match handle.read(&mut buf) {
                        Ok(_) => {
                            if buf[0] != 1 {
                                debug!("Killing thread.");
                                return;
                            }
                            match keys_reference.write() {
                                Ok(mut keys) => {
                                    // if the keys are the same, kill the thread
                                    let mut kill = true;
                                    let mut count = 0;
                                    let mut vector = Vec::new();
                                    for i in 3..8 {
                                        count += buf[i];
                                        vector.push(buf[i]);
                                    }

                                    let mut keys_add = Vec::new();
                                    let mut keys_remove = Vec::new();

                                    for i in 0..5 {
                                        let new_key = vector[i];
                                        if new_key > 0 {
                                            kill = false;

                                            if !keys.contains(&new_key) {
                                                keys_add.push(new_key);
                                            }
                                        }

                                        if i < keys.len() {
                                            let old_key = keys[i];
                                            kill = false;
                                            if !vector.contains(&old_key) {
                                                keys_remove.push(old_key);
                                            }
                                        }
                                    }

                                    for key in keys_remove {
                                        let callback = {
                                            match keyup_callback_reference.read() {
                                                Ok(callback) => callback,
                                                Err(err) => {
                                                    error!("Failed to get keyup callback: {}", err);
                                                    return;
                                                }
                                            }
                                        };
                                        match callback.as_ref() {
                                            Some(callback) => {
                                                callback(Key::from(key));
                                                keys.retain(|&x| x != key);
                                            }
                                            None => {}
                                        }
                                    }

                                    for key in keys_add {
                                        let callback = {
                                            match keydown_callback_reference.read() {
                                                Ok(callback) => callback,
                                                Err(err) => {
                                                    error!(
                                                        "Failed to get keydown callback: {}",
                                                        err
                                                    );
                                                    return;
                                                }
                                            }
                                        };
                                        match callback.as_ref() {
                                            Some(callback) => {
                                                callback(Key::from(key));
                                                keys.push(key);
                                            }
                                            None => {}
                                        }
                                    }

                                    if kill {
                                        // Fn key on Mac returns 1, 0, 0, 0, 0, 0, 0, 0
                                        // Which is the same as the default state
                                        // So we need to ignore these key presses
                                        if count > 0 {
                                            debug!("Killing thread");
                                            return;
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to lock keys: {}", e);
                                    return;
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to read: {}", e);
                        }
                    }
                }
            });
        }
    }
}

fn hash(string: String) -> u32 {
    let mut hash: u32 = 0;
    for c in string.chars() {
        hash = (hash as u32).wrapping_mul(31).wrapping_add(c as u32);
    }
    return hash;
}

#[tauri::command]
pub fn list_interfaces() -> Vec<String> {
	let interfaces = get_interfaces();
	let mut names: Vec<String> = Vec::new();
	for interface in interfaces {
		names.push(format!("({}) {}:{}", interface.id, interface.manufacturer, interface.product));
	}
	return names;
}

#[tauri::command]
pub fn list_interfaces_id() -> Vec<u32> {
    let interfaces = get_interfaces();
    let mut ids: Vec<u32> = Vec::new();
    for interface in interfaces {
        ids.push(interface.id);
    }
    return ids;
}

#[tauri::command]
pub fn list_interfaces_name() -> Vec<String> {
    let interfaces = get_interfaces();
    let mut names: Vec<String> = Vec::new();
    for interface in interfaces {
        names.push(format!("{}:{}", interface.manufacturer, interface.product));
    }
    return names;
}

pub fn get_interface_by_id(id: u32) -> Option<Interface> {
    let interfaces = get_interfaces();
    for interface in interfaces {
        if interface.id == id {
            return Some(interface);
        }
    }
    return None;
}

pub fn get_interface_by_name(name: String) -> Option<Interface> {
    let interfaces = get_interfaces();
    for interface in interfaces {
        if format!("{}:{}", interface.manufacturer, interface.product) == name {
            return Some(interface);
        }
    }
    return None;
}

pub fn get_interfaces() -> Vec<Interface> {
    let mut interfaces: Vec<Interface> = Vec::new();
    for device in API.device_list() {
        let manufacturer = device.manufacturer_string().unwrap_or("Unknown");
        let product = device.product_string().unwrap_or("Unknown");
        let serial = device.serial_number().unwrap_or("Unknown");

        let combo = manufacturer.to_owned() + &product + &serial;
        let id = hash(combo);

        let mut new_interface = Interface {
            id: id,
            manufacturer: manufacturer.to_owned(),
            product: product.to_owned(),
            serial: serial.to_owned(),
            keys: Arc::new(RwLock::new(Vec::new())),
            keydown_callback: Arc::new(RwLock::new(None)),
            keyup_callback: Arc::new(RwLock::new(None)),
            handles: RwLock::new(Vec::new()),
        };

        // check if the interface already exists
        let mut exists = false;
        for interface in interfaces.iter_mut() {
            if interface == &new_interface {
                match device.open_device(&API) {
                    Ok(handle) => match interface.handles.write() {
                        Ok(mut h) => {
                            h.push(handle);
                        }
                        Err(e) => {
                            error!("Failed to get handles: {}", e);
                        }
                    },
                    Err(e) => {
                        error!("Failed to open device: {}", e);
                    }
                }
                exists = true;
                break;
            }
        }

        if !exists {
            match device.open_device(&API) {
                Ok(handle) => match new_interface.handles.write() {
                    Ok(mut h) => {
                        h.push(handle);
                    }
                    Err(e) => {
                        error!("Failed to get handles: {}", e);
                    }
                },
                Err(e) => {
                    error!("Failed to open device: {}", e);
                }
            }
            interfaces.push(new_interface);
        }
    }
    return interfaces;
}
