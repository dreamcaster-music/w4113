import { useEffect, useState } from "react";
import "../../globals.css";
import { debug } from "tauri-plugin-log-api";
import Frame from "../components/Frame";
import { event, invoke } from "@tauri-apps/api";

function Settings(props: { visible: boolean }) {
	const [configJson, setConfigJson] = useState({});
	const [showConfig, setShowConfig] = useState(false);

	const [hostOption, setHostOption] = useState([]);
	const [host, setHost] = useState("");

	const [outputDeviceOption, setOutputDeviceOption] = useState([]);
	const [outputDevice, setOutputDevice] = useState("");

	const [inputDeviceOption, setInputDeviceOption] = useState([]);
	const [inputDevice, setInputDevice] = useState("");

	const [outputStreamOption, setOutputStreamOption] = useState([]);
	const [outputStream, setOutputStream] = useState("");

	const [inputStreamOption, setInputStreamOption] = useState([]);
	const [inputStream, setInputStream] = useState("");

	// Called when the component is first mounted
	useEffect(() => {
		invoke("list_hosts").then((hosts: any) => {
			setHostOption(hosts);
		});
	}, []);

	// Called when the host changes
	useEffect(() => {
		debug("Setting host to " + host);
		invoke("set_host", { name: host }).then((response: any) => {
			debug(response);
			invoke("list_output_devices").then((devices: any) => {
				setOutputDeviceOption(devices);
			});
			invoke("list_input_devices").then((devices: any) => {
				setInputDeviceOption(devices);
			});
		});
	}, [host]);

	useEffect(() => {
		debug("Setting output device to " + outputDevice);
		invoke("set_output_device", { name: outputDevice }).then((response: any) => {
			debug(response);
			invoke("list_output_streams").then((streams: any) => {
				setOutputStreamOption(streams);
			});
		});
	}, [outputDevice]);

	useEffect(() => {
		debug("Setting input device to " + inputDevice);
		invoke("set_input_device", { name: inputDevice }).then((response: any) => {
			debug(response);
			invoke("list_input_streams").then((streams: any) => {
				setInputStreamOption(streams);
			});
		});
	}, [inputDevice]);

	return (
		<>
			<style>
				{css}
			</style>
			<Frame x={100} y={50} className="noselect" width={"700px"} height={"auto"} title={"Audio Settings"} visible={props.visible} refreshCallback={() => {
				invoke("list_hosts").then((hosts: any) => {
					setHostOption(hosts);
				});
			}}>
				<div className="option">
					Host:
					<select className="host-select"
						onChange={(event) => {
							setHost(event.target.value);
						}}>
						{hostOption.map((host: any) => {
							return <option>{host}</option>;
						})}
						<option>default</option>
					</select>
				</div>

				<div className="option">
					Output Device:
					<select className="output-device-select" onChange={(event) => {
						setOutputDevice(event.target.value);
					}}>
						{outputDeviceOption.map((device: any) => {
							return <option>{device}</option>;
						})}
						<option>default</option>
					</select>
				</div>

				<div className="option">
					Input Device:
					<select className="input-device-select" onChange={(event) => {
						setInputDevice(event.target.value);
					}}>
						{inputDeviceOption.map((device: any) => {
							return <option>{device}</option>;
						})}
						<option>default</option>
					</select>
				</div>

				<div className="option">
					Output Stream:
					<select className="output-stream-select">
						{outputStreamOption.map((stream: any) => {
							return <option>{stream}</option>;
						})}
						<option>default</option>
					</select>
				</div>

				<div className="option">
					Input Stream:
					<select className="input-stream-select">
						{inputStreamOption.map((stream: any) => {
							return <option>{stream}</option>;
						})}
						<option>default</option>
					</select>
				</div>

				<div className="option">
					Output Buffer: <input type="number" defaultValue="1024" className="output-buffer-input" />
					Input Buffer: <input type="number" defaultValue="1024" className="input-buffer-input" />
				</div>

				<button className="config-button" onClick={() => {
					invoke("config_json").then((json: any) => {
						setConfigJson(json);
					});
					setShowConfig(!showConfig);
				}}>Config</button>
			</Frame >
			<Frame x={100} y={50} className="noselect" width={"700px"} height={"auto"} title={"Config"} visible={showConfig} refreshCallback={() => {
				invoke("config_json").then((json: any) => {
					setConfigJson(json);
				});
			}}>
				{JSON.stringify(configJson)}
			</Frame >
		</>
	);
}

const css = `

.option {
	margin: 20px;
	display: flex;
	flex-direction: row;

	white-space: nowrap;

	font: 900 18px var(--font-mono);
	color: var(--accent);

	height: 20px;
	max-height: 20px;
}

.option select {
	width: 100%;
}

.option input {
	width: 100%;
}

input[type="number"] {
	-webkit-appearance: textfield;
	-moz-appearance: textfield;
	appearance: textfield;
}

input[type=number]::-webkit-inner-spin-button,
input[type=number]::-webkit-outer-spin-button {
	-webkit-appearance: none;
}

.config-button {
	margin: 20px;
	display: flex;
	flex-direction: row;

	white-space: nowrap;

	font: 900 18px var(--font-mono);
	color: var(--accent);

	height: 20px;
	max-height: 20px;

	border: 1px solid var(--accent);
	background-color: var(--background);
}

`;

export default Settings;