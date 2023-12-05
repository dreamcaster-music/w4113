import React from "react";
import ReactDOM from "react-dom/client";
import Console from "./console/Console";
import { attachConsole, debug } from "tauri-plugin-log-api";
import { WebviewWindow } from "@tauri-apps/api/window";
import { emit, listen } from "@tauri-apps/api/event";
import { Update } from "./bindings/ConfigUpdate";

attachConsole();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<Console />
);