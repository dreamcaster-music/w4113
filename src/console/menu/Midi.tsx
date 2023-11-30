import { useEffect, useState } from "react";
import Frame from "../components/Frame";
import { invoke } from "@tauri-apps/api";
import { debug } from "tauri-plugin-log-api";

function Midi(props: { visible: boolean }) {
	const [midiInputOptions, setMidiInputOptions] = useState([]);
	const [midiSelections, setMidiSelections] = useState(["Select a MIDI device"]);


	let midiDeviceSelectList = []

	for (let i = 0; i < midiSelections.length; i++) {
		let defaultValue = midiSelections[i];
		midiDeviceSelectList.push(
			<>
				<div className="settings">
					MIDI Device {i + 1}:
					<select onChange={(event) => {
						let devices = midiInputOptions;
						devices[i] = event.target.value;

						let midiSelectionsCopy = midiSelections;
						midiSelectionsCopy[i] = event.target.value;
						setMidiSelections(midiSelectionsCopy);
					}}>
						{midiInputOptions.map((value, index) => {
							return (
								<option value={value} defaultValue={defaultValue}>{value}</option>
							)
						})}
					</select>

					{i < midiSelections.length - 1 ? null :
						<button className="remove" onClick={() => {
							removeOption(i);
						}}>
							-
						</button>
					}
				</div>
			</>
		)
	}

	function addOption() {
		setMidiSelections(midiSelections.concat([""]));
	}

	function removeOption(index: number) {
		let selectionsCopy = [];
		for (let i = 0; i < midiSelections.length; i++) {
			if (i !== index) {
				selectionsCopy.push(midiSelections[i]);
			}
		}
		setMidiSelections(selectionsCopy);
	}

	useEffect(() => {
		invoke("midi_list").then((response: any) => {
			debug(response);
			setMidiInputOptions(response);
		});
	}, []);

	return (
		<>
			<style>
				{css}
			</style>
			<Frame title="MIDI Settings" className="noselect" visible={props.visible} x={100} y={100} width="700px" height="300px">
				{midiDeviceSelectList}
				<button className="add" onClick={() => {
					addOption();
				}}>
					+
				</button>

			</Frame>
		</>
	);
}

const css = `

.settings {
	margin: 20px;
	display: flex;
	flex-direction: row;

	white-space: nowrap;

	font: 900 18px var(--font-mono);
	color: var(--accent);

	height: 20px;
	max-height: 20px;
}

.settings select {
	width: 100%;
}

.remove {
	margin-left: 20px;
	text-align: center;
	width: 30px;

	font: 900 18px var(--font-mono);
	color: var(--accent-alt);

	background-color: transparent;
	border: 1px solid var(--accent-alt);
}

.remove:hover {
	color: var(--accent);
	border: 1px solid var(--accent);
}

.add {
	margin-left: 20px;
	margin-right: 20px;
	margin-bottom: 20px;
	text-align: center;
	width: calc(100% - 40px);

	font: 900 18px var(--font-mono);
	color: var(--accent-alt);

	background-color: transparent;
	border: 1px solid var(--accent-alt);
}

.add:hover {
	color: var(--accent);
	border: 1px solid var(--accent);
}

`;

export default Midi;