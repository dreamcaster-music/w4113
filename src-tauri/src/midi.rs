//! midi.rs
//! 
//! Module for handling midi devices

use midir;

use midir::{Ignore, MidiInput, MidiOutput};

/// ## `midi_list() -> Vec<String>`
/// 
/// Returns a list of midi devices
/// 
/// ### Returns
/// 
/// `Vec<String>` - A vector of strings containing the midi devices
pub fn midi_list() -> Vec<String> {
    //list midi devices
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
    return vec![test.to_string()];
}
