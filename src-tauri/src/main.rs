// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;
mod granulizer;
mod interface;
mod midi;
mod tv;

use lazy_static::lazy_static;
use log::{debug, error, LevelFilter};
use serde::Serialize;
use std::sync::{Mutex, RwLock, Arc};
use tauri::{AppHandle, Manager};
use tauri_plugin_log::LogTarget;

use crate::config::Config;

static CONFIG_FILE: &str = "public/config.json";
static CONFIG_ROOT: &str = "public/config/";

// The current configuration
lazy_static! {
    static ref APP_HANDLE: Mutex<Option<AppHandle>> = Mutex::new(None);
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::empty());
    static ref CONSOLE_WINDOW: Mutex<Option<tauri::Window>> = Mutex::new(None);
    static ref TV_WINDOW: Mutex<Option<tauri::Window>> = Mutex::new(None);
}

pub fn try_emit<S: Serialize + Clone>(event: &str, data: S) {
	match APP_HANDLE.lock() {
		Ok(mut app_handle_mutex) => {
			if let Some(app_handle) = app_handle_mutex.as_mut() {
				let _ = app_handle
					.emit_all(event, data);
			}
		}
		Err(e) => {
			error!("Error locking APP_HANDLE: {}", e);
		}
	}
}

#[tauri::command]
fn run() {
    debug!("Svelte finished loading. Making windows visible.");
    match CONSOLE_WINDOW.lock() {
        Ok(mut console_window_mutex) => {
            if let Some(console_window) = console_window_mutex.as_mut() {
                match console_window.show() {
                    Ok(()) => {
                        debug!("Successfully showed console window");
                    }
                    Err(e) => {
                        debug!("Error showing console window: {}", e);
                    }
                }
                console_window.on_window_event(|event| match event {
                    tauri::WindowEvent::Destroyed => {
                        debug!("Console window closed");
                        std::process::exit(0);
                    }
                    _ => {}
                });
            }
        }
        Err(e) => {
            debug!("Error locking CONSOLE_WINDOW: {}", e);
        }
    }

	match audio::STRIPS.write() {
		Ok(mut strips) => {

			let mut sine = audio::plugin::SineGenerator::new();
			sine.add_freq(440.0, 1.0);
			let input = audio::Input::Generator(Arc::new(Mutex::new(sine)));
			let mut new_strip = audio::Strip::new(input, audio::Output::Stereo(0, 1));
			//strips.push(new_strip);
		}

		Err(e) => {
			error!("Error locking STRIPS: {}", e);
		}
	}

	let _ = audio::audio_thread();
}

/// ## `main()`
///
/// The main function.
/// This function is called when the program is run. This should not be used to initialize the program, that should be done in `event_loop`.
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            println!("Starting w4113");

            // set CONSOLE_WINDOW and TV_WINDOW
            let console_window = app.get_window("console").unwrap();
            let tv_window = app.get_window("tv").unwrap();

            match CONSOLE_WINDOW.lock() {
                Ok(mut console_window_mutex) => {
                    *console_window_mutex = Some(console_window);
                }
                Err(e) => {
                    debug!("Error locking CONSOLE_WINDOW: {}", e);
                }
            }

            match TV_WINDOW.lock() {
                Ok(mut tv_window_mutex) => {
                    *tv_window_mutex = Some(tv_window);
                }
                Err(e) => {
                    debug!("Error locking TV_WINDOW: {}", e);
                }
            }

            // set APP_HANDLE
            match APP_HANDLE.lock() {
                Ok(mut app_handle_mutex) => {
                    *app_handle_mutex = Some(app.handle());
                }
                Err(e) => {
                    error!("Error locking APP_HANDLE: {}", e);
                }
            }

            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout, LogTarget::Webview])
                .level(LevelFilter::Debug)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            run,
            audio::set_host,
            audio::list_hosts,
            audio::list_output_devices,
            audio::set_output_device,
            audio::list_output_streams,
            audio::set_output_stream,
            audio::set_input_buffer_size,
            audio::list_input_devices,
            audio::set_input_device,
            audio::list_input_streams,
            audio::set_input_stream,
            audio::set_output_buffer_size,
			audio::audio_thread,
			audio::play_sample,
            midi::midi_list,
			interface::list_interfaces,
			interface::list_interfaces_id,
            interface::list_interfaces_name,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
