import "./styles.css";
import App from "./console/App.svelte";
import { attachConsole, debug } from "tauri-plugin-log-api";
import { invoke } from "@tauri-apps/api";

const detach = await attachConsole();

invoke("run").then(() => {

});

const app = new App({
	// @ts-ignore
	target: document.getElementById("app"),
});

export default app;
