import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { debug } from "tauri-plugin-log-api";

import { ConsoleMessage } from "./bindings/ConsoleMessage";
import { appWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";

/**
 * ## App()
 * 
 * Main React component for the app
 * 
 * @returns w4113 app element
 */
function App() {
	const [consoleOutput, setConsoleOutput] = useState<ConsoleMessage[]>([]);
	const [textSize, setTextSize] = useState<number>(16);

	/**
	 * ## outputMessage(output: ConsoleMessage)
	 * 
	 * Outputs a message to the console
	 * 
	 * ### Parameters
	 * @param output - The message to output to the console
	 * 
	 * ### Returns
	 * @returns void
	 */
	function outputMessage(output: ConsoleMessage) {
		setConsoleOutput((prev) => [...prev, output]);
	}

	/**
	 * ## runCommand(executeCommand: string)
	 * 
	 * Runs the command specified by executeCommand
	 * 
	 * ### Parameters
	 * @param executeCommand - The command to run
	 * 
	 * ### Returns
	 * @returns void
	 */
	function runCommand(executeCommand: string) {
		debug("Command invoked from w4113 console:\n " + executeCommand);

		// Split command into array of strings
		let split = executeCommand.split(" ");

		// format command
		let command = split[0];
		let args: string[] = [];
		let arg = "";
		let section = "";
		let inQuotes = false;
		for (let i = 1; i < split.length; i++) {
			let arg = split[i];
			if (inQuotes) {
				if (arg.endsWith('"')) {
					inQuotes = false;
					section += " " + arg.substring(0, arg.length - 1);
					args.push(section.substring(1));
					section = "";
				} else {
					section += " " + arg;
				}
			} else {
				if (arg.startsWith('"')) {
					inQuotes = true;
					section = arg;
				} else {
					args.push(arg);
				}
			}
		}

		switch (command) {
			case "help":
				/*
				 * Help command
				 * Usage: help [command]
				 * 
				 * Displays list of available commands
				 * Displays help for command -- requires command
				 */
				outputMessage({ kind: "Console", message: "Available commands: help, clear, about, host, output, input, config, sine, reave, exit" });
				break;
			case "clear":
				/*
				 * Clear command
				 * Usage: clear
				 * 
				 * Clears the console
				 */
				setConsoleOutput([{ kind: "User", message: "clear" }]);
				break;
			case "about":
				/*
				 * About command
				 * Usage: about
				 * 
				 * Displays about message
				 */
				outputMessage({ kind: "Console", message: "(w4113 pre-alpha) by dreamcaster: written by ronin beaver and wesley studt" });
				outputMessage({ kind: "Console", message: "prepare to get vaporized meatbags" });
				break;
			case "host":
				/*
				 * Host command
				 * Usage: host [list|select|clear] [host]
				 * 
				 * list: list all hosts
				 * select: select host -- requires host
				 * clear: clear selected host
				 */

				break;
			case "output":
				/*
				 * Output command
				 * Usage: output [list]
				 * 
				 * list: list all outputs
				 */

				break;
			case "input":
				/*
				 * Output command
				 * Usage: output [list]
				 * 
				 * list: list all outputs
				 */

				break;
			case "config":
				/*
				 * Config command
				 * Usage: config [show|load|save] [filename]
				 * 
				 * show: show current config
				 * load: load config from file -- requires filename
				 * save: save config to file -- requires filename
				 */

				break;
			case "exit":

				break;
			case "reave":
				outputMessage({ kind: "Console", message: "You have been reaved." });
				break;
			case "sine":
				appWindow.emit("audio-test", { payload: args.join(" ") });
				break;
			case "":
				break;
			default:
				outputMessage({ kind: "Error", message: "Command not found: " + command });
				break;
		}
	}

	/**
	 * ## handleInput(event: any)
	 * 
	 * Handles input from the console
	 * 
	 * ### Parameters
	 * @param event - The event that triggered the input
	 * 
	 * ### Returns
	 * @returns void
	 */
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
		invoke("run").then((response) => {
			debug("Result from run: " + response);
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
						return <p key={index} style={{ fontSize: textSize + "px" }} className="console-user"> {"> " + message.message}</p>;
					case "Console":
						return <p key={index} style={{ fontSize: textSize + "px" }} className="console-command">{message.message}</p>;
					case "Error":
						return <p key={index} style={{ fontSize: textSize + "px" }} className="console-error">{message.message}</p>;
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
