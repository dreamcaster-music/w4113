<script lang="ts">
	import { fade } from "svelte/transition";
	import { trace } from "tauri-plugin-log-api";
	import "../../globals.css";

	const handleSize = 12;

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
</script>

{#if visible}
	<div
		class="frame mono bg-black"
		style="width: {width}px; height: {height}px; left: {left}px; top: {top}px;"
		id={createUniqueId()}
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
	</div>
{/if}

<svelte:window on:mouseup={onMouseUp} on:mousemove={onMouseMove} />

<style>
	.frame {
		position: absolute;
		border: 1px solid var(--accent);
	}

	.frame-body {
		overflow: scroll;
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
</style>
