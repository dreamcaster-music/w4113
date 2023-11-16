import "./TV.css";

import { useState } from "react";
import { BasicVisualizer } from "./visualizers/BasicVisualizer";

function TV() {
	const [visualizer, setVisualizer] = useState<String>("Basic");

	let visualizerComponent;

	switch (visualizer) {
		case "Basic":
			visualizerComponent = <BasicVisualizer />
			break;
		default:
			visualizerComponent = <BasicVisualizer />
			break;
	}

	return (
		<>
			<div className="container" data-tauri-drag-region>
				{visualizerComponent}
			</div>
		</>
	)
}

export default TV;