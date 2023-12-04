import React from "react";
import ReactDOM from "react-dom/client";
import Console from "./console/Console";
import { attachConsole, debug } from "tauri-plugin-log-api";
import { WebviewWindow } from "@tauri-apps/api/window";
import { Provider } from 'react-redux';
import { emit, listen } from "@tauri-apps/api/event";
import { Update } from "./bindings/ConfigUpdate";

attachConsole();

let configJSON: any = {};
let onChangeMap: Map<string, (key: string, value: any) => void> = new Map();

async function stateEventLoop() {
	const unlisten = await listen("rust-state-update", (event) => {
		let payload = event.payload as Update;
		let key = payload.key;
		let value = payload.value;

		configJSON[key] = value;
	});
}

stateEventLoop();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<Console />
);

function onChange(key: string, onChange: (key: string, value: any) => void) {
	onChangeMap.set(key, onChange);
}

function getConfigOr(key: string, defaultValue: any) {
	let value = configJSON[key];

	if (value == undefined) {
		value = defaultValue;
		setConfig(key, defaultValue);
	}

	return value;
}

function setConfig(key: string, value: any) {
	let originalValue = configJSON[key];
	if (value == originalValue) return;
	onChangeMap.get(key)?.(key, value);
	emit("react-state-update", { key: key, value: value });
}

export { getConfigOr, setConfig, configJSON, onChange };