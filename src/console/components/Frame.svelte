<script lang="ts">
	import { fade } from "svelte/transition";
	import { trace } from "tauri-plugin-log-api";
	import "../../globals.css";

	const handleSize = 14;

	let id = createUniqueId();
	export let title: string;
	export let width: number | string = "";
	export let height: number | string = "";
	export let left: number = 100;
	export let top: number = 100;
	export let visible: boolean = false;
	let dragging: boolean = false;

	function createUniqueId() {
		let id =
			Math.random().toString(36).substring(2, 15) +
			Math.random().toString(36).substring(2, 15);

		let element = document.getElementById(id);

		if (element) {
			return createUniqueId();
		}

		return id;
	}

	function close() {
		trace(title + ": closing.");
		visible = false;
	}

	function drag(event: MouseEvent) {
		trace(title + ": started dragging.");
		dragging = true;
	}

	function onMouseUp() {
		if (dragging) {
			trace(title + ": stopped dragging.");
			dragging = false;
		}
	}

	function onMouseMove(event: MouseEvent) {
		if (dragging) {
			left += event.movementX;
			top += event.movementY;
		}
	}

	function bringToFront() {
		// get all frames
		let frames = document.getElementsByClassName("frame");
		let count = frames.length;
		let max = 10 + count;

		// set all frames to z-index 10
		for (let i = 0; i < count; i++) {
			let currentZIndex = parseInt(
				(frames[i] as HTMLElement).style.zIndex,
			);
			(frames[i] as HTMLElement).style.zIndex = currentZIndex - 1 + "";
		}

		// set this frame to
		let frame = document.getElementById(id);

		if (frame) {
			(frame as HTMLElement).style.zIndex = max + "";
		}
	}

	function getHighestFrame(): number {
		// get all frames
		let frames = document.getElementsByClassName("frame");
		let count = frames.length;
		let max = 0;

		for (let i = 0; i < count; i++) {
			let zIndex = parseInt((frames[i] as HTMLElement).style.zIndex);

			if (zIndex > max) {
				max = zIndex;
			}
		}

		return max;
	}
</script>

{#if visible}
	<button
		class="frame mono bg-black z-10"
		style="width: {width}px; height: {height}px; left: {left}px; top: {top}px; z-index: {getHighestFrame() +
			1};"
		{id}
		on:mousedown={bringToFront}
		transition:fade={{ delay: 0, duration: 100 }}
	>
		<button class="title-bar none" on:mousedown={drag} on:dblclick={close}>
			<button class="none" on:click={close}>
				<img
					src="close.svg"
					data-target="close"
					class="handle close"
					draggable="false"
					alt="close"
					style="width: {handleSize}px; height: {handleSize}px;"
				/>
			</button>
			{title}
		</button>
		<div class="frame-body">
			<slot />
		</div>
	</button>
{/if}

<svelte:window on:mouseup={onMouseUp} on:mousemove={onMouseMove} />

<style>
	.frame {
		position: fixed;
		border: 1px solid var(--accent);
	}

	.frame-body {
		overflow: hidden;
	}

	.title-bar {
		display: flex;
		flex-direction: row;
		width: calc(100% - 20px);
		border-bottom: 1px solid var(--border);

		color: var(--accent);
		font: 900 16px var(--font-mono);
		padding-top: 10px;
		padding-left: 10px;
		padding-right: 10px;

		height: 24px;
		max-height: 24px;
	}

	.handle {
		position: absolute;
		right: 14px;
		top: 14px;
		margin: 0px;
		opacity: 0.5;
	}

	.handle:hover {
		opacity: 1;
	}
</style>
