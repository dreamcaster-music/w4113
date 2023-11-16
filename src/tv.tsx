import React from "react";
import ReactDOM from "react-dom/client";
import { attachConsole } from "tauri-plugin-log-api";
import { WebviewWindow } from "@tauri-apps/api/window";
import TV from "./tv/TV";

attachConsole();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<TV />
);
