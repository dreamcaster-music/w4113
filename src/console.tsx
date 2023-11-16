import React from "react";
import ReactDOM from "react-dom/client";
import Console from "./console/Console";
import { attachConsole } from "tauri-plugin-log-api";
import { WebviewWindow } from "@tauri-apps/api/window";

attachConsole();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<Console />
);
