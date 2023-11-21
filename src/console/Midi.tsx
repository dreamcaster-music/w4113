import { useState } from "react";
import "./Midi.css";
import { debug } from "tauri-plugin-log-api";
import { invoke } from "@tauri-apps/api";

// get tauri win

function Midi() {
	const [midi, setMidi] = useState(false);

	function listenKeys(e: KeyboardEvent) {
		invoke("");
	}

	function onClick() {
		let midi_new = !midi;
		setMidi(midi_new);

		if (midi_new) {
			document.addEventListener("keydown", listenKeys);
			invoke("midi_keyboard_on");
		} else {
			document.removeEventListener("keydown", listenKeys);
			invoke("midi_keyboard_off");
		}
	}

	return (
		<>
			<button className="midi" onClick={onClick} style={{ backgroundColor: midi ? "greenyellow" : "white" }}>
				{midi ? "Midi: On" : "Midi: Off"}
			</button>
		</>
	);
}

export default Midi;