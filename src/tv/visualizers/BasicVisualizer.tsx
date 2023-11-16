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
			let height = bar * 100;
			return <div className="bar" key={index} style={{ top: "calc(50% - " + (height / 2) + "px)", height: "1px", left: index / barsLength * 100 + "%" }}></div>
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