import "./BasicVisualizer.css";
import { emit, listen } from '@tauri-apps/api/event'


import { useEffect, useState } from "react";
import { debug } from "tauri-plugin-log-api";

let resolution = 1000;

function BasicVisualizer() {
	// bars is an array of floats
	const [bars, setBars] = useState([0.0]);

	const loadbars = listen('loadbars', (event) => {
		console.log("loadbars");

		setBars(event.payload as [number]);
	});

	let barsLength = bars.length;
	let barsComponent = bars.map((bar, index) => {
		if (bars[index] != 0) {
			if (index % 8 != 0) {
				return <></>;
			}
			let height = bar * 50;
			let heightAbs = Math.abs(height);
			if (height > 0) {
				return <div className="bar" key={index} style={{ bottom: "50%", height: heightAbs + "px", left: index / barsLength * 100 + "%" }}></div>
			} else {
				return <div className="bar" key={index} style={{ top: "50%", height: heightAbs + "px", left: index / barsLength * 100 + "%" }}></div>
			}
		}
	});


	// Runs once on component load
	useEffect(() => {

	}, []);

	return (
		<>
			{barsComponent}
		</>
	)
}

export { BasicVisualizer };