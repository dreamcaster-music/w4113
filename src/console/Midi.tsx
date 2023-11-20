import "./Midi.css";

// display a simple midi keyboard from c4 to f5

function Key(props: { note: string, octave: number, sharp: boolean }) {
	return (
		<div className={"key " + props.note + " octave" + props.octave + (props.sharp ? " sharp" : " natural")} />
	);
}

const notes = [["c", 0, false], ["c", 0, true], ["d", 0, false], ["d", 0, true], ["e", 0, false], ["f", 0, false], ["f", 0, true], ["g", 0, false], ["g", 0, true], ["a", 0, false], ["a", 0, true], ["b", 0, false],
["c", 1, false], ["c", 1, true], ["d", 1, false], ["d", 1, true], ["e", 1, false], ["f", 1, false]];

function Midi() {
	let notesComponents = [];
	for (const note in notes) {
		let pitch = note[0];
		let octave = note[1] as any as number;
		let sharp = note[2] as any as boolean;

		notesComponents.push(<Key note={pitch} octave={octave} sharp={sharp} />);
	}

	return (
		<>
			<div className="piano">
				{notesComponents}
			</div>
		</>
	);
}

export default Midi;