use midir;
use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput, MidiOutput};

pub fn midi_list() -> Vec<String> {
    //list midi devices
    let mut midi_in = MidiInput::new("midir reading input").unwrap();
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("midir writing output").unwrap();
    let mut midi_out_ports = midi_out.ports();
    let mut midi_in_ports = midi_in.ports();
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
