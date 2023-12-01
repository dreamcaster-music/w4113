import React from "react";
import ReactDOM from "react-dom/client";
import Console from "./console/Console";
import { attachConsole } from "tauri-plugin-log-api";
import { WebviewWindow } from "@tauri-apps/api/window";
import { Provider } from 'react-redux'
import store from "./store";

attachConsole();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<Provider store={store}>
		<Console />
	</Provider>,
);
