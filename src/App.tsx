import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { debug } from "tauri-plugin-log-api";

import { ConsoleMessage } from "./bindings/ConsoleMessage";

function App() {
	const [consoleOutput, setConsoleOutput] = useState<ConsoleMessage[]>([]);

	function outputMessage(output: ConsoleMessage) {
		setConsoleOutput((prev) => [...prev, output]);
	}

	function outputStr(type: string, output: string) {
		// Create new paragraph element
		const outputDiv = document.querySelector(".console-output");
		const outputP = document.createElement("p");

		// Add text to paragraph
		outputP.innerHTML = output;
		outputP.className = "console-error";
		if (type === "User") {
			outputP.className = "console-user";
		} else if (type === "Console") {
			outputP.className = "console-command";
		} else if (type === "Error") {
			outputP.className = "console-error";
		}

		// Append paragraph to output div
		outputDiv?.appendChild(outputP);

		// scroll to bottom
		outputDiv?.scrollTo(0, outputDiv.scrollHeight);
	}

	function runCommand(executeCommand: string) {
		debug("Command invoked from w4113 console:\n " + executeCommand);

		// Split command into array of strings
		let split = executeCommand.split(" ");

		let command = split[0];
		let args = split.slice(1);

		switch (split[0]) {
			case "help":
				outputMessage({ kind: "Console", message: "Available commands: help, clear, about, host, device, config, exit" });
				break;
			case "clear":
				setConsoleOutput([{ kind: "User", message: "clear" }]);
				break;
			case "about":
				outputMessage({ kind: "Console", message: "(w4113 pre-alpha) by dreamcaster: written by ronin beaver and wesley studt" });
				outputMessage({ kind: "Console", message: "prepare to get vaporized meatbags" });
				break;
			case "host":

				break;
			case "device":

				break;
			case "config":
				invoke("tauri_call", { command: "config", args: args }).then((response) => {
					outputMessage(response as ConsoleMessage);
				});
				break;
			case "exit":

				break;
			default:
				outputMessage({ kind: "Error", message: "Command not found: " + command });
				break;
		}
	}

	// Function to handle enter key press in console input
	function handleInput(event: any) {
		if (event.key === "Enter") {
			// Add command to output
			outputMessage({ kind: "User", message: event.target.value });

			// Send command to command handler
			runCommand(event.target.value);

			// Clear input
			event.target.value = "";
		}
	}

	// Runs once on app load
	useEffect(() => {
		debug("React App finished loading, now calling Tauri.")
		invoke("tauri_init").then((response) => {
			debug("Result from tauri_init" + response);
			outputMessage({ kind: "Console", message: "Welcome to w4113. Type 'help' for a list of commands." });
		});
	}, []);

	// Runs every time consoleOutput changes
	useEffect(() => {
		// scroll to bottom
		const outputDiv = document.querySelector(".console-output");
		outputDiv?.scrollTo(0, outputDiv.scrollHeight);
	}, [consoleOutput]);


	let output =
		<>
			{consoleOutput.map((message, index) => {
				switch (message.kind) {
					case "User":
						return <p key={index} className="console-user"> {"> " + message.message}</p>;
					case "Console":
						return <p key={index} className="console-command">{message.message}</p>;
					case "Error":
						return <p key={index} className="console-error">{message.message}</p>;
					default:
						return <></>;
				}
			})}
		</>;

	return (
		<>
			<div className="app">
				<div className="container" data-tauri-drag-region>
					<div className="console">
						<div className="console-output">
							{output}
						</div>
						<input className="console-input" type="text" placeholder="Enter command" onKeyDown={handleInput} />
					</div>
				</div>
			</div>
		</>
	);
}

export default App;
