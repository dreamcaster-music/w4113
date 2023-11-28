import { trace } from "tauri-plugin-log-api";
import "../globals.css";
import "./Frame.css";
import { useEffect, useState } from "react";

function Frame(props: { width?: string, height?: string, title?: string, visible?: boolean, className?: any, children?: any }) {
	const handleSize = 12;
	const [position, setPosition] = useState({ x: 0, y: 0, dx: 0, dy: 0, dragging: false });
	const [size, setSize] = useState({ width: "auto", height: "auto" });
	const [visible, setVisible] = useState(props.visible);

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

	return (
		<>
			{(visible) && (
				<div className={"frame " + props.className} style={{ width: size.width, height: size.height, left: position.x, top: position.y }} onMouseDown={onMouseDown} onMouseUp={onMouseUp} onMouseMove={onMouseMove}>
					<div className="title-bar">
						<img src="close.svg" data-target="close" className="handle close" style={{ width: handleSize + "px", height: handleSize + "px" }} draggable="false" />
						<img src="drag.svg" data-target="drag" className="handle drag" style={{ width: handleSize + "px", height: handleSize + "px" }} draggable="false" />
						{props.title || "Untitled"}
					</div>
					<div className="frame-body">
						{props.children}
					</div>
				</div>
			)}
		</>
	);
}

export default Frame;