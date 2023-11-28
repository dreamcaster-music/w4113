import { useEffect, useState } from "react";
import "../globals.css";
import "./Settings.css";
import "./Frame.css";
import { debug } from "tauri-plugin-log-api";
import Frame from "./Frame";
import { event, invoke } from "@tauri-apps/api";

function Settings(props: { visible?: boolean }) {
	const [hostOption, setHostOption] = useState([]);
	const [host, setHost] = useState("default");

	const [outputDeviceOption, setOutputDeviceOption] = useState([]);
	const [outputDevice, setOutputDevice] = useState("default");

	const [inputDeviceOption, setInputDeviceOption] = useState([]);
	const [inputDevice, setInputDevice] = useState("default");

	const [outputStreamOption, setOutputStreamOption] = useState([]);
	const [outputStream, setOutputStream] = useState("default");

	const [inputStreamOption, setInputStreamOption] = useState([]);
	const [inputStream, setInputStream] = useState("default");


	// Called when the component is first mounted
	useEffect(() => {
		invoke("list_hosts").then((hosts: any) => {
			setHostOption(hosts);
		});
	}, []);

	// Called when the host changes
	useEffect(() => {
		invoke("host_select", { host: host }).then((response: any) => {
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
		invoke("set_output_device", { name: outputDevice }).then((response: any) => {
			debug(response);
			invoke("list_output_streams").then((streams: any) => {
				setOutputStreamOption(streams);
			});
		});
	}, [outputDevice]);

	useEffect(() => {
		invoke("set_input_device", { name: inputDevice }).then((response: any) => {
			debug(response);
			invoke("list_input_streams").then((streams: any) => {
				setInputStreamOption(streams);
			});
		});
	}, [inputDevice]);

	let defaultOption = <></>;
	// defaultOption = <option>default</option>;

	return (
		<Frame className="panel settings noselect" width={"700px"} height={"auto"} title={"Audio Settings"} visible={props.visible}>
			<div className="option">
				Host:
				<select className="host-select" onChange={(event) => {
					setHost(event.target.value);
				}}>
					{hostOption.map((host: any) => {
						return <option>{host}</option>;
					})}
					{defaultOption}
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
					{defaultOption}
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
					{defaultOption}
				</select>
			</div>

			<div className="option">
				Output Stream:
				<select className="output-stream-select">
					{outputStreamOption.map((stream: any) => {
						return <option>{stream}</option>;
					})}
					{defaultOption}
				</select>
			</div>

			<div className="option">
				Input Stream:
				<select className="input-stream-select">
					{inputStreamOption.map((stream: any) => {
						return <option>{stream}</option>;
					})}
					{defaultOption}
				</select>
			</div>

			<div className="option">
				Output Buffer: <input type="number" className="output-buffer-input" />
				Input Buffer: <input type="number" className="input-buffer-input" />
			</div>
		</Frame>
	);
}

export default Settings;