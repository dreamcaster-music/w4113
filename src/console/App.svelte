<script lang="ts">
	import AudioSettings from "./AudioSettings.svelte";
	import KeyboardSettings from "./KeyboardSettings.svelte";
	import MidiSettings from "./MidiSettings.svelte";

	let dockWidth = 68;
	let dockIconSize = 30;

	let audioVisible: boolean = false;
	let keyboardVisible: boolean = false;
	let midiVisible: boolean = false;
	let sampleVisible: boolean = false;

	import "../globals.css";
	import Sampler from "./Sampler.svelte";
	import Tempo from "./Tempo.svelte";
</script>

<main class="container w-full h-full" data-tauri-drag-region>
	<Tempo />
	<AudioSettings visible={audioVisible} />
	<KeyboardSettings visible={keyboardVisible} />
	<MidiSettings visible={midiVisible} />
	<Sampler visible={sampleVisible} />

	<div class="dock border-r border-accent" style="width: {dockWidth}px;">
		<button
			class="dock-icon none"
			on:click={() => {
				audioVisible = !audioVisible;
			}}
			style="width: {dockIconSize}px; height: {dockIconSize}px; margin-left: {dockWidth /
				2 -
				dockIconSize / 2}px; margin-right: {dockWidth / 2 -
				dockIconSize / 2}px;"
		>
			<img
				src="settings.svg"
				alt="Audio Settings"
				style="width: {dockIconSize}px; height: {dockIconSize}px;"
			/>
		</button>
		<button
			class="dock-text-icon none"
			on:click={() => {
				keyboardVisible = !keyboardVisible;
			}}
			style="width: {dockIconSize}px; height: {dockIconSize}px; margin-left: {dockWidth /
				2 -
				dockIconSize / 2}px; margin-right: {dockWidth / 2 -
				dockIconSize / 2}px;"
		>
			Key
		</button>
		<button
			class="dock-text-icon none"
			on:click={() => {
				midiVisible = !midiVisible;
			}}
			style="width: {dockIconSize}px; height: {dockIconSize}px; margin-left: {dockWidth /
				2 -
				dockIconSize / 2}px; margin-right: {dockWidth / 2 -
				dockIconSize / 2}px;"
		>
			MIDI
		</button>
		<button
			class="dock-text-icon none"
			on:click={() => {
				sampleVisible = !sampleVisible;
			}}
			style="width: {dockIconSize}px; height: {dockIconSize}px; margin-left: {dockWidth /
				2 -
				dockIconSize / 2}px; margin-right: {dockWidth / 2 -
				dockIconSize / 2}px;"
		>
			RSS7
		</button>
	</div>
</main>

<style>
	.dock {
		top: 0;
		left: 0;
		padding-top: 28px;
		position: absolute;
		height: 100%;
		background-color: var(--dark);
		overflow: hidden;
	}

	.dock-icon {
		margin-top: 5px;
		margin-bottom: 5px;

		opacity: 0.5;
	}

	.dock-text-icon {
		background-color: transparent;
		border: none;

		color: var(--accent);
		font: 900 16px var(--mono);

		text-align: left;

		margin: auto;
		width: 100%;
		height: var(--dock-icon-size);

		text-align: center;
		vertical-align: middle;
		line-height: 30px;
		opacity: 0.5;
	}

	.dock-icon:hover {
		opacity: 1;
	}

	.dock-text-icon:hover {
		opacity: 1;
	}
</style>
