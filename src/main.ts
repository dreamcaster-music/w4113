import { invoke } from "@tauri-apps/api";
import { attachConsole } from "tauri-plugin-log-api";
import App from "./console/App.svelte";
import "./styles.css";

async function run() {
	const detach = await attachConsole();

	invoke("run").then(() => {

	});
}

run();

const app = new App({
	// @ts-ignore
	target: document.getElementById("app"),
});

export default app;
