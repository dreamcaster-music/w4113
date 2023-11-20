import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./Console.css";
import { debug, error } from "tauri-plugin-log-api";

import { ConsoleMessage } from "../bindings/ConsoleMessage";
import { appWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";
import { FreqMessage } from "../bindings/FreqMessage";

/**
 * ## App()
 * 
 * Main React component for the app
 * 
 * @returns w4113 app element
 */
function Console() {
	const [confirmExit, setConfirmExit] = useState<boolean>(false);
	const [commandHistory, setCommandHistory] = useState<string[]>([]);
	const [commandHistoryIndex, setCommandHistoryIndex] = useState<number>(-1);
	const [commandCurrent, setCommandCurrent] = useState<string>("");
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

	function strValue(message: ConsoleMessage): string {
		let content = "";
		for (let i = 0; i < message.message.length; i++) {
			content += message.message[i];
			if (i < message.message.length - 1) {
				content += "\n";
			}
		}
		return content;
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
		// Replace “ and ” with "
		executeCommand = executeCommand.replaceAll("“", "\"");
		executeCommand = executeCommand.replaceAll("”", "\"");

		// Add command to history
		setCommandHistory((prev) => [executeCommand, ...prev]);
		setCommandHistoryIndex(-1);
		setCommandCurrent("");

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

		debug("Command " + command + " with args [" + args + "] invoked from console");

		switch (command) {
			case "help":
				/*
				 * Help command
				 * Usage: help [command]
				 * 
				 * Displays list of available commands
				 * Displays help for command -- requires command
				 */
				outputMessage({ kind: "Console", message: ["Available commands: help, clear, about, host, output, input, config, sine, reave, exit"] });
				break;
			case "clear":
				/*
				 * Clear command
				 * Usage: clear
				 * 
				 * Clears the console
				 */
				setConsoleOutput([{ kind: "User", message: ["clear"] }]);
				break;
			case "about":
				/*
				 * About command
				 * Usage: about
				 * 
				 * Displays about message
				 */
				outputMessage({ kind: "Console", message: ["(w4113 alpha) by dreamcaster: written by ronin beaver and wesley studt"] });
				outputMessage({ kind: "Console", message: ["prepare to get vaporized meatbags"] });
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
				if (args.length < 1) {
					outputMessage({ kind: "Error", message: ["Not enough arguments for host command."] });
					outputMessage({ kind: "Error", message: ["Usage: host [list|select|clear] [host]"] });
					break;
				}

				let hostCommand = args[0];
				switch (hostCommand) {
					case "list":
						invoke("host_list").then((response) => {
							debug("Result from host list: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					case "select":
						if (args.length < 2) {
							outputMessage({ kind: "Error", message: ["Not enough arguments for select host command."] });
							outputMessage({ kind: "Error", message: ["Usage: host select [host]"] });
							break;
						}
						let selectHost = args[1];
						invoke("host_select", { host: selectHost }).then((response) => {
							debug("Result from host select: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					default:
						outputMessage({ kind: "Error", message: ["Invalid host command: " + hostCommand] });
						outputMessage({ kind: "Error", message: ["Usage: host [list|select|clear] [host]"] });
						break;
				}
				break;
			case "output":
				/*
				 * Output command

				 * Usage:
				 * output list
				 * 		list all outputs
				 * 
				 * output select <output>
				 * 		select output
				 * 
				 * output stream show
				 * 		show available stream configurations
				 * 
				 * output stream set <# of channels> <sample rate> <buffer size>
				 * 		set stream configuration
				 * 
				 * list: list all outputs
				 */
				if (args.length < 1) {
					outputMessage({ kind: "Error", message: ["Not enough arguments for output command."] });
					outputMessage({ kind: "Error", message: ["Usage: output [list|select|stream]"] });
					break;
				}

				let outputCommand = args[0];
				switch (outputCommand) {
					case "list":
						invoke("output_list").then((response) => {
							debug("Result from output list: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					case "select":
						if (args.length < 2) {
							outputMessage({ kind: "Error", message: ["Not enough arguments for select output command."] });
							outputMessage({ kind: "Error", message: ["Usage: output select [output]"] });
							break;
						}
						let selectOutput = args[1];
						invoke("output_select", { output: selectOutput }).then((response) => {
							debug("Result from output select: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					case "stream":
						if (args.length < 2) {
							outputMessage({ kind: "Error", message: ["Not enough arguments for stream command."] });
							outputMessage({ kind: "Error", message: ["Usage: output stream [show|set]"] });
							break;
						}
						let streamCommand = args[1];
						switch (streamCommand) {
							case "show":
								invoke("output_stream_show").then((response) => {
									debug("Result from output stream show: " + strValue(response as ConsoleMessage));
									outputMessage(response as ConsoleMessage);
								});
								break;
							case "set":
								if (args.length < 5) {
									outputMessage({ kind: "Error", message: ["Not enough arguments for set stream command."] });
									outputMessage({ kind: "Error", message: ["Usage: output stream set [channels] [sample rate] [buffer size]"] });
									break;
								}
								let numChannels = parseInt(args[2]);
								let sampleRate = parseInt(args[3]);
								let bufferSize = parseInt(args[4]);
								invoke("output_stream_set", { channels: numChannels, samples: sampleRate, bufferSize: bufferSize }).then((response) => {
									debug("Result from stream channel set: " + strValue(response as ConsoleMessage));
									outputMessage(response as ConsoleMessage);
								});
								break;
							default:
								outputMessage({ kind: "Error", message: ["Invalid stream command: " + streamCommand] });
								outputMessage({ kind: "Error", message: ["Usage: output stream [show|set]"] });
								break;
						}
						break;
					default:
						outputMessage({ kind: "Error", message: ["Invalid output command: " + outputCommand] });
						outputMessage({ kind: "Error", message: ["Usage: output [list]"] });
						break;
				}
				break;
			case "input":
				/*
				 * Input command
				 * 
				 * Usage:
				 * input list
				 * 		list all inputs
				 * 
				 * input select <input>
				 * 		select input
				 * 
				 * list: list all inputs
				 * 
				 * select: select input
				 * 		requires input
				 */

				if (args.length < 1) {
					outputMessage({ kind: "Error", message: ["Not enough arguments for input command."] });
					outputMessage({ kind: "Error", message: ["Usage: input [list|select]"] });
					break;
				}

				let inputCommand = args[0];
				switch (inputCommand) {
					case "list":
						invoke("input_list").then((response) => {
							debug("Result from input list: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					case "select":
						if (args.length < 2) {
							outputMessage({ kind: "Error", message: ["Not enough arguments for select input command."] });
							outputMessage({ kind: "Error", message: ["Usage: input select [input]"] });
							break;
						}
						let selectInput = args[1];
						invoke("input_select", { input: selectInput }).then((response) => {
							debug("Result from input select: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					case "stream":
						if (args.length < 2) {
							outputMessage({ kind: "Error", message: ["Not enough arguments for stream command."] });
							outputMessage({ kind: "Error", message: ["Usage: input stream [show|set]"] });
							break;
						}
						let streamCommand = args[1];
						switch (streamCommand) {
							case "show":
								invoke("input_stream_show").then((response) => {
									debug("Result from input stream show: " + strValue(response as ConsoleMessage));
									outputMessage(response as ConsoleMessage);
								});
								break;
							case "set":
								if (args.length < 5) {
									outputMessage({ kind: "Error", message: ["Not enough arguments for set stream command."] });
									outputMessage({ kind: "Error", message: ["Usage: input stream set [channels] [sample rate] [buffer size]"] });
									break;
								}
								let numChannels = parseInt(args[2]);
								let sampleRate = parseInt(args[3]);
								let bufferSize = parseInt(args[4]);
								invoke("input_stream_set", { channels: numChannels, samples: sampleRate, bufferSize: bufferSize }).then((response) => {
									debug("Result from stream channel set: " + strValue(response as ConsoleMessage));
									outputMessage(response as ConsoleMessage);
								});
								break;
							default:
								outputMessage({ kind: "Error", message: ["Invalid stream command: " + streamCommand] });
								outputMessage({ kind: "Error", message: ["Usage: input stream [show|set]"] });
								break;
						}
						break;
					default:
						outputMessage({ kind: "Error", message: ["Invalid input command: " + inputCommand] });
						outputMessage({ kind: "Error", message: ["Usage: input [list|select]"] });
						break;
				}
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
				if (args.length < 1) {
					outputMessage({ kind: "Error", message: ["Not enough arguments for config command."] });
					outputMessage({ kind: "Error", message: ["Usage: config [show|load|save] [filename]"] });
					break;
				}

				let configCommand = args[0];

				switch (configCommand) {
					case "show":
						invoke("config_show").then((response) => {
							debug("Result from config show: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					case "load":
						if (args.length < 2) {
							outputMessage({ kind: "Error", message: ["Not enough arguments for load config command."] });
							outputMessage({ kind: "Error", message: ["Usage: config load [filename]"] });
							break;
						}
						let loadConfigFilename = args[1];
						invoke("config_load", { filename: loadConfigFilename }).then((response) => {
							debug("Result from config load: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					case "save":
						if (args.length < 2) {
							outputMessage({ kind: "Error", message: ["Not enough arguments for save config command."] });
							outputMessage({ kind: "Error", message: ["Usage: config save [filename]"] });
							break;
						}
						let saveConfigFilename = args[1];
						invoke("config_save", { filename: saveConfigFilename }).then((response) => {
							debug("Result from config save: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					default:
						outputMessage({ kind: "Error", message: ["Invalid config command: " + configCommand] });
						outputMessage({ kind: "Error", message: ["Usage: config [show|load|save] [filename]"] });
						break;
				}
				break;
			case "exit":
				/*
				 * Exit command
				 * Usage: exit
				 * 
				 * Exits the application
				 */
				if (confirmExit) {
					outputMessage({ kind: "Console", message: ["Exiting..."] });
					setTimeout(() => {
						invoke("confirm_exit").then((response) => {

						});
					}, 500);
				} else {
					setConfirmExit(true);
					invoke("exit").then((response) => {
						debug("Result from exit: " + strValue(response as ConsoleMessage));
						outputMessage(response as ConsoleMessage);
					});
				}
				break;
			case "reave":
				outputMessage({ kind: "Console", message: ["You have been reaved."] });
				break;
			case "sine":
				/*
				 * Sine command
				 * Usage: sine [frequency] [amplitude] [duration]
				 * 
				 * Plays a sine wave of frequency [frequency] Hz, amplitude [amplitude], and duration [duration] seconds
				 */
				if (args.length < 3) {
					outputMessage({ kind: "Error", message: ["Not enough arguments for sine command."] });
					outputMessage({ kind: "Error", message: ["Usage: sine [frequency] [amplitude] [duration]"] });
					break;
				}
				let freq = parseFloat(args[0]);
				let amp = parseFloat(args[1]);
				let dur = parseFloat(args[2]);
				invoke("sine", { frequency: freq, amplitude: amp, duration: dur }).then((response) => {
					debug("Result from sine: " + strValue(response as ConsoleMessage));
					outputMessage(response as ConsoleMessage);
				});
				break;
			case "midi":
				/*
				 * Midi command
				 * Usage: midi [list|start|stop] [device]
				 * 
				 * list: list available midi devices
				 * start: start midi input -- requires device
				 * stop: stop midi input
				 */
				if (args.length < 1) {
					outputMessage({ kind: "Error", message: ["Not enough arguments for midi command."] });
					outputMessage({ kind: "Error", message: ["Usage: midi [list|start|stop] [device]"] });
					break;
				}

				let midiCommand = args[0];
				switch (midiCommand) {
					case "list":
						invoke("midi_list").then((response) => {
							debug("Result from midi list: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					case "start":
						if (args.length < 2) {
							outputMessage({ kind: "Error", message: ["Not enough arguments for start midi command."] });
							outputMessage({ kind: "Error", message: ["Usage: midi start [device]"] });
							break;
						}
						let device_name = args[1] as string;
						invoke("midi_start", { deviceName: device_name }).then((response) => {
							debug("Result from midi start: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					case "stop":
						invoke("midi_stop").then((response) => {
							debug("Result from midi stop: " + strValue(response as ConsoleMessage));
							outputMessage(response as ConsoleMessage);
						});
						break;
					default:
						outputMessage({ kind: "Error", message: ["Invalid midi command: " + midiCommand] });
						outputMessage({ kind: "Error", message: ["Usage: midi [list|start|stop] [device]"] });
						break;
				};
				break;
			case "":
				break;
			default:
				outputMessage({ kind: "Error", message: ["Command not found: " + command] });
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
			outputMessage({ kind: "User", message: [event.target.value] });

			// Send command to command handler
			runCommand(event.target.value);

			// Clear input
			event.target.value = "";
		} else if (event.key === "ArrowUp") {
			if (commandHistoryIndex < commandHistory.length - 1) {
				if (commandHistoryIndex === -1) {
					setCommandCurrent(event.target.value);
				}

				setCommandHistoryIndex(commandHistoryIndex + 1);
				event.target.value = commandHistory[commandHistoryIndex + 1];
			}
		} else if (event.key === "ArrowDown") {
			if (commandHistoryIndex > 0) {
				setCommandHistoryIndex(commandHistoryIndex - 1);
				event.target.value = commandHistory[commandHistoryIndex - 1];
			} else {
				event.target.value = commandCurrent;
			}
		}
	}

	// Runs once on app load
	useEffect(() => {
		debug("React App finished loading, now calling Tauri.")
		invoke("run").then((response) => {
			debug("Result from run: " + response);
			outputMessage({ kind: "Console", message: ["Welcome to w4113. Type 'help' for a list of commands."] });
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
				let content = "";
				for (let i = 0; i < message.message.length; i++) {
					content += message.message[i];
					if (i < message.message.length - 1) {
						content += "\n";
					}
				}

				switch (message.kind) {
					case "User":
						return <p key={index} style={{ fontSize: textSize + "px" }} className="console-user"> {"> " + content}</p>;
					case "Console":
						return <p key={index} style={{ fontSize: textSize + "px" }} className="console-command">{content}</p>;
					case "Error":
						return <p key={index} style={{ fontSize: textSize + "px" }} className="console-error">{content}</p>;
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

export default Console;
