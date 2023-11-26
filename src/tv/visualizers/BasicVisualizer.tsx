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

	let windowHeight = window.innerHeight;
	let windowWidth = window.innerWidth;
	let barWidth = windowWidth / 480;

	let barsLength = bars.length;
	let barsComponent = bars.map((bar, index) => {
		if (bars[index] != 0) {
			if (index % 8 != 0) {
				return <></>;
			}
			let height = bar * ((windowHeight - 500) / 2);
			let heightAbs = Math.abs(height);

			// rainbow red to green to blue left to right
			let red = Math.floor(Math.sin(index / barsLength * Math.PI) * 127 + 128);
			let green = Math.floor(Math.sin(index / barsLength * Math.PI + 2) * 127 + 128);
			let blue = Math.floor(Math.sin(index / barsLength * Math.PI + 4) * 127 + 128);
			let color = "rgb(" + red + "," + green + "," + blue + ")";


			if (height > 0) {
				return <div className="bar" key={index} style={{ bottom: "50%", height: heightAbs + "px", width: barWidth + "px", left: index / barsLength * 100 + "%", backgroundColor: color }}></div>
			} else {
				return <div className="bar" key={index} style={{ top: "50%", height: heightAbs + "px", width: barWidth + "px", left: index / barsLength * 100 + "%", backgroundColor: color }}></div>
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