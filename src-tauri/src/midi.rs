//! midi.rs
//!
//! Module for handling midi devices

use std::sync::RwLock;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use midir;

use midir::{Ignore, MidiInput, MidiOutput};

use lazy_static::lazy_static;

use crate::audio;


/// ## `midi_list() -> Vec<String>`
///
/// Returns a list of midi devices
///
/// ### Returns
///
/// `Vec<String>` - A vector of strings containing the midi devices
#[tauri::command]
pub fn midi_list() -> Vec<String> {
    //list midi devices
    let mut midi_in = match MidiInput::new("midir reading input") {
		Ok(midi_in) => midi_in,
		Err(err) => {
			debug!("Error: {}", err);
			return vec!["Error".to_string()];
		}
	};
    midi_in.ignore(Ignore::None);
    let midi_out = match MidiOutput::new("midir writing output") {
		Ok(midi_out) => midi_out,
		Err(err) => {
			debug!("Error: {}", err);
			return vec!["Error".to_string()];
		}
	};
    let _midi_out_ports = midi_out.ports();
    let midi_in_ports = midi_in.ports();
    let mut midi_devices = Vec::new();
    for i in 0..midi_in_ports.len() {
        midi_devices.push(format!(
            "{}: {:?}\n",
            i,
            match midi_in.port_name(&midi_in_ports[i]) {
				Ok(name) => name,
				Err(err) => {
					debug!("Error: {}", err);
					return vec!["Error".to_string()];
				}
			}
        ));
    }
    
	if midi_devices.len() == 0 {
		midi_devices.push("No midi devices found".to_string());
	}
	midi_devices
}


struct Note {
	amp: f32,
	freq: f32,
	velocity: f32,
	sample_clock: Option<u64>,
}

impl Note {
	fn key(&self) -> f32 {
		self.freq
	}
}

lazy_static! {
    static ref NOTE: RwLock<Vec<Note>> = RwLock::new(Vec::new());
}

static NOTE_SPEED: f32 = 0.002;

pub fn callback(state: &audio::State) -> audio::Sample {
	
    let mut notes = NOTE.write().unwrap();
    let mut output = 0.0;
    for note in notes.iter_mut() {
		let sample_start = match note.sample_clock {
			Some(x) => x,
			None => {
				note.sample_clock = Some(state.sample_clock);
				state.sample_clock
			}
		};

		let sample = (state.sample_clock as i128 - sample_start as i128) as f32 * note.freq * 2.0 * std::f32::consts::PI / state.sample_rate as f32;
		let sample = sample.sin() * note.velocity * note.amp;

		if note.amp > 1.0 {
			note.amp = 1.0;
		}
		if note.amp < 1.0 && note.amp > 0.0 {
			note.amp -= NOTE_SPEED;
		}
		
		output += sample;
    }
	// remove notes where amp <= 0
	for mut i in 0..notes.len() {
		if i >= notes.len() {
			break;
		}
		if notes[i].amp <= 0.0 {
			notes.remove(i);
		}
	}


	audio::Sample::Stereo(output, output)

}

fn midi_callback(stamp: u64, message: &[u8], _: &mut ()) {
    let status = message[0];
    let note = message[1];
    let velocity = message[2];

    let freq = 440.0 * 2.0f32.powf((note as f32 - 69.0) / 12.0);

    match status {
        144 => {
			match velocity {
				0 => {
					debug!("Note off: {} {} {}", note, velocity, freq);
					// subtract note amp by 0.1
					let mut note = NOTE.write().unwrap();
					for i in 0..note.len() {
						if note[i].key() == freq {
							note[i].amp -= 0.01;
						}
					}
				}
				_ => {
					debug!("Note on: {} {} {}", note, velocity, freq);
					NOTE.write().unwrap().push(Note {
						amp: 1.0,
						freq: freq,
						velocity: velocity as f32 / 127.0,
						sample_clock: None,
					});
				}
			}
        }
        128 => {
			debug!("Note off: {} {} {}", note, velocity, freq);
			// subtract note amp by 0.1
			let mut note = NOTE.write().unwrap();
			for i in 0..note.len() {
				if note[i].key() == freq {
					note[i].amp -= 0.1;
				}
			}
        }
        _ => {}
    }

    debug!("{}: {:?} (len = {})", stamp, message, message.len());
}

pub fn midi_start(device_name: String) -> Result<(), String> {
    //start midi device
    let mut midi_in = MidiInput::new("midir reading input").unwrap();
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("midir writing output").unwrap();
    let _midi_out_ports = midi_out.ports();
    let midi_in_ports = midi_in.ports();
    let mut test = String::new();
    for i in 0..midi_in_ports.len() {
        test.push_str(&format!(
            "{}: {:?}\n",
            i,
            midi_in.port_name(&midi_in_ports[i]).unwrap()
        ));
    }
    debug!("{}", test);
    let in_port = &midi_in_ports[0];
    let out_port = &midi_out.ports()[0];
    let in_port_name = midi_in.port_name(in_port).unwrap();
    let out_port_name = midi_out.port_name(out_port).unwrap();
    debug!("Opening connection");
    let conn_in = midi_in.connect(in_port, "midir-read-input", midi_callback, ());

    let conn_in = match conn_in {
        Ok(conn_in) => conn_in,
        Err(err) => {
            debug!("Error: {}", err);
            return Err(err.to_string());
        }
    };

    let conn_out = midi_out.connect(out_port, "midir-write-output").unwrap();
    debug!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );
    let mut input = String::new();
    loop {
        // sleep for 1 second
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    debug!("Closing connection");
    conn_in.close();
    conn_out.close();
    debug!("Connection closed. Goodbye!");

    Ok(())
}

// turns the computer keyboard into a midi keyboard
// a = c
// w = c#
// s = d
// e = d#
// d = e
// f = f
// t = f#
// g = g
// y = g#
// h = a
// u = a#
// j = b
// k = c
// o = c#
// l = d
// p = d#
// ; = e
// ' = f
/*
pub fn midi_keyboard_thread() {
    // create midi port
    let midi_out = MidiOutput::new("midir writing output").unwrap();

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?
        }
    };

    // listen for keyboard up/down events

}
*/