import { useEffect, useState } from "react";
import Frame from "../components/Frame";
import { invoke } from "@tauri-apps/api";
import { debug } from "tauri-plugin-log-api";
import { Key } from "../../bindings/Key";

class Keybind {
	key: Key;
	action: () => void;
}

function keyFromString(keyInput: string) {
	let key: Key = "Unknown";
	switch (keyInput) {
		case "A":
			key = "A";
			break;
		case "B":
			key = "B";
			break;
		case "C":
			key = "C";
			break;
		case "D":
			key = "D";
			break;
		case "E":
			key = "E";
			break;
		case "F":
			key = "F";
			break;
		case "G":
			key = "G";
			break;
		case "H":
			key = "H";
			break;
		case "I":
			key = "I";
			break;
		case "J":
			key = "J";
			break;
		case "K":
			key = "K";
			break;
		case "L":
			key = "L";
			break;
		case "M":
			key = "M";
			break;
		case "N":
			key = "N";
			break;
		case "O":
			key = "O";
			break;
		case "P":
			key = "P";
			break;
		case "Q":
			key = "Q";
			break;
		case "R":
			key = "R";
			break;
		case "S":
			key = "S";
			break;
		case "T":
			key = "T";
			break;
		case "U":
			key = "U";
			break;
		case "V":
			key = "V";
			break;
		case "W":
			key = "W";
			break;
		case "X":
			key = "X";
			break;
		case "Y":
			key = "Y";
			break;
		case "Z":
			key = "Z";
			break;
		case "1":
			key = "Num1";
			break;
		case "2":
			key = "Num2";
			break;
		case "3":
			key = "Num3";
			break;
		case "4":
			key = "Num4";
			break;
		case "5":
			key = "Num5";
			break;
		case "6":
			key = "Num6";
			break;
		case "7":
			key = "Num7";
			break;
		case "8":
			key = "Num8";
			break;
		case "9":
			key = "Num9";
			break;
		case "0":
			key = "Num0";
			break;
		case "Enter":
			key = "Enter";
			break;
		case "Escape":
			key = "Escape";
			break;
		case "Backspace":
			key = "Backspace";
			break;
		case "Tab":
			key = "Tab";
			break;
		case "Space":
			key = "Space";
			break;
		case "-":
			key = "Minus";
			break;
		case "=":
			key = "Equals";
			break;
		case "[":
			key = "LeftBracket";
			break;
		case "]":
			key = "RightBracket";
			break;
		case "\\":
			key = "Backslash";
			break;
		case "#":
			key = "NonUsHash";
			break;
		case ";":
			key = "Semicolon";
			break;
		case "'":
			key = "Apostrophe";
			break;
		case "`":
			key = "Grave";
			break;
		case ",":
			key = "Comma";
			break;
		case ".":
			key = "Period";
			break;
		case "/":
			key = "Slash";
			break;
		case "CapsLock":
			key = "CapsLock";
			break;
		case "F1":
			key = "F1";
			break;
		case "F2":
			key = "F2";
			break;
		case "F3":
			key = "F3";
			break;
		case "F4":
			key = "F4";
			break;
		case "F5":
			key = "F5";
			break;
		case "F6":
			key = "F6";
			break;
		case "F7":
			key = "F7";
			break;
		case "F8":
			key = "F8";
			break;
		case "F9":
			key = "F9";
			break;
		case "F10":
			key = "F10";
			break;
		case "F11":
			key = "F11";
			break;
		case "F12":
			key = "F12";
			break;
		default:
			key = "Unknown";
			break;
	}
	return key;
}

function Keyboard(props: { visible: boolean }) {
	const [keyboardOptions, setKeyboardOptions] = useState([]);
	const [keybinds, setKeybinds] = useState([]);

	useEffect(() => {
		invoke("list_interfaces_name").then((interfaces: any) => {
			debug(interfaces);
			setKeyboardOptions(interfaces);
		});
	}, []);

	function addOption() {
		setKeybinds(keybinds.concat([new Keybind()]));
	}

	return (
		<>
			<style>
				{css}
			</style>
			<Frame title="Keyboard Settings" className="noselect" visible={props.visible} x={100} y={150} width="700px" height="300px">
				<div className="option">
					Keyboard:
					<select className="keyboard-select" onChange={(event) => {
						invoke("start_interface_name", { name: event.target.value }).then((response: any) => {
							debug(response);
						}
						);
					}}>
						{keyboardOptions.map((keyboard: any) => {
							return <option>{keyboard}</option>;
						})}
					</select>
				</div>
				<div className="keybinds">
					{keybinds.map((keybind: Keybind, index: number) => {
						return (
							<div className="keybinds">
								<div className="keybind">
									<input className="keybind-input" type="text" placeholder="Key" onChange={(event) => {
										keybind.key = keyFromString(event.target.value);
									}} />
									Action:
									<select className="keybind-action">

									</select>
								</div>
							</div>
						);
					})}
				</div>
				<button className="add" onClick={() => {
					addOption();
				}}>+</button>
			</Frame>
		</>
	);
}

const css = `

.keybind {
	color: var(--accent);

	margin-left: 20px;
	margin-right: 20px;
	margin-bottom: 10px;

	display: flex;
	flex-direction: row;
}

.keybind input {
	margin-right: 10px;
	width: 75px;
}

.keybind select {
	margin-left: 10px;
	width: 100%;
}

`;

export default Keyboard;