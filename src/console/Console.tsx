import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "../globals.css";
import { debug, error } from "tauri-plugin-log-api";

import { ConsoleMessage } from "../bindings/ConsoleMessage";
import { appWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";
import { FreqMessage } from "../bindings/FreqMessage";
import Frame from "./components/Frame";
import Settings from "./menu/Settings";
import Midi from "./menu/Midi";
import Keyboard from "./menu/Keyboard";
import Terminal from "./menu/Terminal";

const dockWidth = 68;
const dockIconSize = 30;

/**
 * ## App()
 * 
 * Main React component for the app
 * 
 * @returns w4113 app element
 */
function Console() {
	const [settingsVisible, setSettingsVisible] = useState(false);
	const [midiVisible, setMidiVisible] = useState(false);
	const [keyboardVisible, setKeyboardVisible] = useState(false);
	const [terminalVisible, setTerminalVisible] = useState(false);

	// Runs once on app load
	useEffect(() => {
		debug("React App finished loading, now calling Tauri.")
		invoke("run").then((response) => {

		});
	}, []);

	return (
		<>
			<style>
				{css}
			</style>

			<div className="app">
				<div className="container" data-tauri-drag-region>
					<Settings visible={settingsVisible} />
					<Midi visible={midiVisible} />
					<Keyboard visible={keyboardVisible} />
					<Terminal visible={terminalVisible} />



					<div className="dock">
						<img src="settings.svg" className="dock-icon" draggable="false" onClick={() => {
							setSettingsVisible(!settingsVisible);
						}} />
						<button className="dock-text-icon" draggable="false" onClick={() => {
							setTerminalVisible(!terminalVisible);
						}}>
							&gt;_
						</button>
						<button className="dock-text-icon" draggable="false" onClick={() => {
							setMidiVisible(!midiVisible);
						}}>
							MIDI
						</button>
						<button className="dock-text-icon" draggable="false" onClick={() => {
							setKeyboardVisible(!keyboardVisible);
						}}>
							Key
						</button>
					</div>
				</div>
			</div >
		</>
	);
}

const css = `

			html,
			body,
			:root,
			.root,
			.container {
				position: absolute;
			height: 100%;
			width: 100%;
			top: 0;
			left: 0;
			margin: 0;
			padding: 0;

			background-color: var(--dark);
			overflow: hidden;
}

			.dock {
				padding-top: 28px;
				position: absolute;
			width: ${dockWidth}px;
			height: 100%;
			background-color: var(--dark);
			overflow: hidden;

			border-right: 1px solid var(--accent);
}

			.console {
				position: absolute;
}

			.dock-icon {
				

				margin-left: ${(dockWidth - dockIconSize) / 2}px;
				margin-right: ${(dockWidth - dockIconSize) / 2}px;

				width: ${dockIconSize}px;
				height: ${dockIconSize}px;

				margin-top: 5px;
				margin-bottom: 5px;

				opacity: 0.5;
			}

			.dock-text-icon {
				background-color: transparent;
				border: none;

				color: var(--accent);
				font: 900 16px var(--mono);
				
				text-align: left;

				margin: auto;
				width: 100%;
				height: ${dockIconSize}px;

				text-align: center;
				vertical-align: middle;
				line-height: 30px;
				opacity: 0.5;
			}

			.dock-icon:hover {
				opacity: 1;
			}

			.dock-text-icon:hover {
				opacity: 1;
			}

			`

export default Console;
