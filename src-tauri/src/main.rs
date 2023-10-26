// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::debug;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// The kind of message that can be used by the frontend. This is also used on the TypeScript side for its own functions.
/// 
/// Serde and TS-RS are used to make this enum available to both Rust and TypeScript.
#[derive(ts_rs::TS, serde::Serialize, serde::Deserialize)]
#[ts(export, export_to = "../src/bindings/MessageKind.ts")]
enum MessageKind {
	/// A message from the user
	User,

	/// A message from the console
	Console,

	/// An error message
	Error
}

/// A message that can be sent to the frontend
/// 
/// This exists so that we can send messages to React and have some extra data along with it. React needs the information of
/// what kind of message it is so that it can be displayed correctly.
/// 
/// Serde and TS-RS are used to make this struct available to both Rust and TypeScript.
#[derive(ts_rs::TS, serde::Serialize, serde::Deserialize)]
#[ts(export, export_to = "../src/bindings/ConsoleMessage.ts")]
struct ConsoleMessage {

	/// The kind of message
	kind: MessageKind,

	/// The message itself
	message: String
}

// Global config variable based on serde_json
// This allows it to be saved and loaded from file
static mut CONFIG: Option<serde_json::Value> = None;

fn main() {
    tauri::Builder::default()
		.plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![

		])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
