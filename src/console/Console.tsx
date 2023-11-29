import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./Console.css";
import { debug, error } from "tauri-plugin-log-api";

import { ConsoleMessage } from "../bindings/ConsoleMessage";
import { appWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";
import { FreqMessage } from "../bindings/FreqMessage";
import Settings from "./Settings";
import Frame from "./ui/Frame";

/**
 * ## App()
 * 
 * Main React component for the app
 * 
 * @returns w4113 app element
 */
function Console() {
	// Runs once on app load
	useEffect(() => {
		debug("React App finished loading, now calling Tauri.")
		invoke("run").then((response) => {

		});
	}, []);

	return (
		<>
			<div className="app">
				<div className="container" data-tauri-drag-region>
					<Settings visible={true} />

					<button onClick={() => {
						// invoke hid_list
						invoke("hid_list").then((response) => {
							//debug(response);
						});
					}} style={{ position: "absolute", left: "50px", bottom: "50px" }}>HID List</button>
				</div>
			</div>
		</>
	);
}

export default Console;
