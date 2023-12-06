import "./styles.css";
import App from "./console/App.svelte";
import { attachConsole } from "tauri-plugin-log-api";

const detach = await attachConsole();

const app = new App({
	// @ts-ignore
	target: document.getElementById("app"),
});

export default app;
