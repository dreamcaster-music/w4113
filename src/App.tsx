import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { debug } from "tauri-plugin-log-api";

function App() {
	const [selectedHost, setSelectedHost] = useState<string>("");

	const outputConsole = (output: string) => {
		const outputDiv = document.querySelector(".console-output");
		const outputP = document.createElement("p");
		outputP.innerHTML = output;
		outputP.className = "console-command";
		outputDiv?.appendChild(outputP);

		// scroll to bottom
		outputDiv?.scrollTo(0, outputDiv.scrollHeight);
	}

	const outputConsoleError = (output: string) => {
		const outputDiv = document.querySelector(".console-output");
		const outputP = document.createElement("p");
		outputP.innerHTML = output;
		outputP.className = "console-error";
		outputDiv?.appendChild(outputP);

		// scroll to bottom
		outputDiv?.scrollTo(0, outputDiv.scrollHeight);
	}

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

		let split = command.split(" ");

		switch (split[0]) {
			case "help":
				outputConsole("Available commands: help, clear, about, host, device");
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
				outputConsole("prepare to get vaporized meatbag");
				break;
			case "host":
				if (split[1] === "list") {
					invoke("host_list").then((result: any) => {
						outputConsole("Available Hosts:");
						for (const host of result) {
							outputConsole("- " + host);
						}
						outputConsole("Use 'host select [hostname]' to select a host.");
					});
				} else if (split[1] === "select") {
					if (split[2]) {
						let hosts = invoke("host_list").then((result: any) => {
							if (result.includes(split[2])) {
								setSelectedHost(split[2]);
								outputConsole("Selected host: " + split[2]);
								outputConsole("Use 'device list' to view available devices for this host.");
							} else {
								outputConsoleError("Host not found.");
							}
						});
					} else {
						outputConsoleError("Invalid arguments for 'host select' command.\nSyntax: host select [hostname]");
					}
				} else {
					outputConsoleError("Invalid arguments for 'host' command.\nSyntax: host [list|select] [hostname]");
				}
				break;
			case "device":
				if (selectedHost === "") {
					outputConsoleError("No host selected. Use 'host select [hostname]' to select a host.");
					break;
				} else {
					if (split[1] === "list") {
						invoke("device_list", { hostname: selectedHost }).then((result: any) => {
							outputConsole("Available Devices:");
							for (const device of result) {
								outputConsole("- " + device);
							}
							outputConsole("Use 'device select [device]' to select a device.");
						});
					} else {
						outputConsoleError("Invalid arguments for 'device' command.\nSyntax: device [list]");
					}
				}
				break;
			default:
				outputConsoleError("Command not found. Type 'help' for a list of commands.");
				break;
		}
	}

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
