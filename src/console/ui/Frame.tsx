import { debug, trace } from "tauri-plugin-log-api";
import "../../globals.css";
import { useEffect, useState } from "react";

function Frame(props: { width?: string, height?: string, x?: number, y?: number, title?: string, visible?: boolean, className?: any, children?: any, refreshCallback?: () => void }) {
	const handleSize = 12;
	const [position, setPosition] = useState({ x: props.x || 0, y: props.y || 0, dx: 0, dy: 0, dragging: false });
	const [size, setSize] = useState({ width: "auto", height: "auto" });
	const [visible, setVisible] = useState(!props.visible);

	function onMouseDown(event: React.MouseEvent<HTMLDivElement, MouseEvent>) {
		if (!visible) return;

		// @ts-ignore
		let target = event.target.dataset.target;

		if (target == "drag") {
			let dx = event.clientX - position.x;
			let dy = event.clientY - position.y;
			setPosition({ x: event.clientX - dx, y: event.clientY - dy, dx: dx, dy: dy, dragging: true });
		} else if (target == "close") {
			setVisible(false);
		} else if (target == "refresh") {
			if (props.refreshCallback) {
				trace((props.title || "Untitled") + ": refresh callback");
				props.refreshCallback();
			}
		}
	}

	function onMouseUp(event: React.MouseEvent<HTMLDivElement, MouseEvent>) {
		if (!position.dragging || !visible) return;
		setPosition({ x: event.clientX - position.dx, y: event.clientY - position.dy, dx: position.dx, dy: position.dy, dragging: false });
	}

	function onMouseMove(event: React.MouseEvent<HTMLDivElement, MouseEvent>) {
		if (!position.dragging || !visible) return;
		setPosition({ x: event.clientX - position.dx, y: event.clientY - position.dy, dx: position.dx, dy: position.dy, dragging: true });
	}

	useEffect(() => {
		if (visible) {
			trace((props.title || "Untitled") + ": showing frame");
		} else {
			trace((props.title || "Untitled") + ": hiding frame");
		}
	}, [visible]);

	useEffect(() => {
		if (position.dragging) {
			trace((props.title || "Untitled") + ": dragging frame");
		} else {
			trace((props.title || "Untitled") + ": stopped dragging frame");
		}
	}, [position.dragging]);

	useEffect(() => {
		setVisible(!visible);
	}, [props.visible]);

	let refreshHandle = <></>
	if (props.refreshCallback) {
		refreshHandle = (
			<img src="refresh.svg" data-target="refresh" className="handle refresh" style={{ width: handleSize * 1.3 + "px", height: handleSize * 1.3 + "px" }} draggable="false" onClick={() => {

			}} />
		);
	}

	return (
		<>
			<style>
				{css}
			</style>
			{(visible) && (
				<div className={"frame mono bg-black " + props.className} style={{ width: size.width, height: size.height, left: position.x, top: position.y }} onMouseDown={onMouseDown} onMouseUp={onMouseUp} onMouseMove={onMouseMove}>
					<div className="title-bar">
						<img src="close.svg" data-target="close" className="handle close" style={{ width: handleSize + "px", height: handleSize + "px" }} draggable="false" />
						<img src="drag.svg" data-target="drag" className="handle drag" style={{ width: handleSize + "px", height: handleSize + "px" }} draggable="false" />
						{props.title || "Untitled"}
						{refreshHandle}
					</div>
					<div className="frame-body" style={{ width: props.width, height: props.height }}>
						{props.children}
					</div>
				</div>
			)}
		</>
	);
}

const css = `

.frame {
	position: absolute;
	border: 1px solid var(--accent);
}

.title-bar {
	display: flex;
	flex-direction: row;

	color: var(--accent);
	font: 900 16px var(--font-mono);
	padding-top: 10px;
	padding-left: 10px;
	padding-right: 10px;

	height: 15px;
	max-height: 15px;
}

.handle {
	padding-top: 2px;
	padding-right: 10px;
	margin: 0px;
	opacity: 0.5;
}

.handle:hover {
	opacity: 1;
}

.refresh {
	margin-left: auto;
}
/*
.drag {
	cursor: move;
}

.close {
	cursor: pointer;
}

.refresh {
	cursor: pointer;
}
*/

`;

export default Frame;