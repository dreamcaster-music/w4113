import { useEffect, useState } from "react";
import Frame from "../components/Frame";
import { invoke } from "@tauri-apps/api";
import { debug } from "tauri-plugin-log-api";

function Terminal(props: { visible: boolean }) {
	const [output, setOutput] = useState(["Welcome to the w4113 console!"]);

	return (
		<>
			<style>
				{css}
			</style>
			<Frame title="Terminal" className="noselect" visible={props.visible} x={100} y={100} width="700px" height="300px">
				<div className="terminal-output">
					{output.map((line) => {
						return (
							<>
								{line}
								<br />
							</>
						);
					})}
					<br />
					<br />
				</div>
				<input className="terminal-input" placeholder="Type a command..." onKeyDown={(e) => {
					if (e.key == "Enter") {
						setOutput([...output, e.currentTarget.value]);
						e.currentTarget.value = "";
					}
				}}>
				</input>
			</Frame>
		</>
	);
}

const css = `

.terminal-output {
	padding: 10px;
	font-size: 12px;
	color: var(--accent);
}

.terminal-input {
	width: ${700 - 20}px;
	position: absolute;
	font-size: 12px;
	padding: 10px;
	bottom: 0;
	background: black;
	color: white;
	border: none;
}

.terminal-input::placeholder {
	color: grey;
}

`;

export default Terminal;