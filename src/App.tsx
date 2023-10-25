import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { debug } from "tauri-plugin-log-api";

function App() {
	const outputAny = (output: any) => {
		switch (output.kind) {
			case "User":
				userConsole(output.message);
				break;
			case "Console":
				outputConsole(output.message);
				break;
			case "Error":
				outputConsoleError(output.message);
				break;
			default:
				break;
		}
	}

	// Output to console
	const outputConsole = (output: string) => {
		const outputDiv = document.querySelector(".console-output");
		const outputP = document.createElement("p");
		outputP.innerHTML = output;
		outputP.className = "console-command";
		outputDiv?.appendChild(outputP);

		// scroll to bottom
		outputDiv?.scrollTo(0, outputDiv.scrollHeight);
	}

	// Output error to console
	const outputConsoleError = (output: string) => {
		const outputDiv = document.querySelector(".console-output");
		const outputP = document.createElement("p");
		outputP.innerHTML = output;
		outputP.className = "console-error";
		outputDiv?.appendChild(outputP);

		// scroll to bottom
		outputDiv?.scrollTo(0, outputDiv.scrollHeight);
	}

	// Output user command to console
	const userConsole = (output: string) => {
		const outputDiv = document.querySelector(".console-output");
		const outputP = document.createElement("p");
		outputP.innerHTML = "> " + output;
		outputP.className = "console-user";
		outputDiv?.appendChild(outputP);

		// scroll to bottom
		outputDiv?.scrollTo(0, outputDiv.scrollHeight);
	}

	const runCommand = (command: string) => {
		debug("Command invoked from w4113 console:\n " + command);

		// Split command into array of strings
		let split = command.split(" ");

		switch (split[0]) {
			case "help":
				outputConsole("Available commands: help, clear, about, host, device, config, exit");
				break;
			case "clear":
				let output = document.querySelector(".console-output");
				output?.remove();
				output = document.createElement("div");
				output.className = "console-output";
				const container = document.querySelector(".console");
				container?.appendChild(output);

				userConsole("clear");
				break;
			case "about":
				outputConsole("w4113.exe version 0.0.1");
				outputConsole("created by: w4113");
				outputConsoleError("prepare to get vaporized meatbag");
				break;
			case "host":
				if (split[1] === "list") {
					invoke("host_list").then((result: any) => {
						outputAny(result);
					});
				} else if (split[1] === "select") {
					let host = split[2];
					if (host !== undefined) {
						invoke("host_select", { host: host }).then((result: any) => {
							outputAny(result);
						});
					} else {
						outputConsoleError("Invalid arguments for 'host' command.\nSyntax: host [list|select] [hostname]");
					}
				} else {
					outputConsoleError("Invalid arguments for 'host' command.\nSyntax: host [list|select] [hostname]");
				}
				break;
			case "device":
				if (split[1] === "list") {
					invoke("device_list").then((result: any) => {
						outputAny(result);
					});
				} else {
					outputConsoleError("Invalid arguments for 'device' command.\nSyntax: device [list]");
				}
				break;
			case "config":
				if (split[1] === "save") {
					if (split[2] !== undefined) {
						invoke("config_save", { path: split[2] }).then((result: any) => {
							outputAny(result);
						});
					} else {
						outputConsoleError("Invalid arguments for 'config' command.\nSyntax: config [save|load]");
					}
				} else if (split[1] === "load") {
					if (split[2] !== undefined) {
						invoke("config_load", { path: split[2] }).then((result: any) => {
							outputAny(result);
						});
					} else {
						outputConsoleError("Invalid arguments for 'config' command.\nSyntax: config [save|load]");
					}
				} else if (split[1] === "show") {
					invoke("config_show").then((result: any) => {
						outputAny(result);
					});
				} else {
					outputConsoleError("Invalid arguments for 'config' command.\nSyntax: config [save|load]");
				}
				break;
			case "exit":
				invoke("exit");
				break;
			default:
				outputConsoleError("Command not found. Type 'help' for a list of commands.");
				break;
		}
	}

	// Function to handle enter key press in console input
	const handleInput = (event: any) => {
		if (event.key === "Enter") {
			// Add command to output
			userConsole(event.target.value);

			// Send command to backend
			runCommand(event.target.value);

			// Clear input
			event.target.value = "";
		}
	}

	return (
		<>
			<div className="app">
				<div className="container" data-tauri-drag-region>
					<div className="console">
						<div className="console-output">
						</div>
						<input className="console-input" type="text" placeholder="Enter command" onKeyDown={handleInput} />
					</div>
				</div>
			</div>
		</>
	);
}

export default App;
