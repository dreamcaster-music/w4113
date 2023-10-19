import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {


	return (
		<>
			<div className="app">
				<div className="container" data-tauri-drag-region>
					<input className="console-input" type="text" placeholder="Enter command" />
				</div>
			</div>
		</>
	);
}

export default App;
