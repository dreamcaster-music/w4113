import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { attachConsole } from "tauri-plugin-log-api";

attachConsole();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<App />
);
