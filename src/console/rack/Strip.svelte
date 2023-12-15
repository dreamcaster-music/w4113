<script lang="ts">
	import { listen } from "@tauri-apps/api/event";
	import { debug } from "tauri-plugin-log-api";
	import ContextMenu from "../components/ContextMenu.svelte";
	import Frame from "../components/Frame.svelte";
	import Effect from "./Effect.svelte";
	import Input from "./Input.svelte";
	import Output from "./Output.svelte";

	const width = 256;
	export let value = 50;
	export let strip_index: number;
	export let strip: any;

	let showInputControls = false;
	let contextMenuLocation = { x: 0, y: 0 };
	let contextMenu: string[] = [];

	$: strip = strip;

	let removeeffect = listen("rust-removeeffect", (payload) => {
		// @ts-ignore
		let index = payload.payload.index;

		strip.chain[index] = null;
		strip.chain = strip.chain;
	});

	let seteffect = listen("rust-seteffect", (payload) => {
		// @ts-ignore
		let pstrip = payload.payload.strip;

		if (pstrip != strip_index) {
			debug("strip mismatch");
			return;
		}

		// @ts-ignore
		let index = payload.payload.index;
		// @ts-ignore
		let effect = payload.payload.effect;

		strip.chain[index] = effect;
		strip.chain = strip.chain;
	});
</script>

{#if contextMenu.length > 0}
	<ContextMenu
		bind:options={contextMenu}
		x={contextMenuLocation.x}
		y={contextMenuLocation.y}
		callback={() => {}}
	/>
{/if}

<div
	class="h-full border-r-2 flex col-auto text-white font-mono"
	style="left: {strip_index *
		width}px; width: {width}px; min-width: {width}px; border-color: gray;"
>
	<div class="flex flex-col">
		<p class="ml-3 mt-3">{strip_index}</p>
		<input
			type="range"
			min="0"
			max="100"
			bind:value
			class="w-4"
			style="--value: {value}%;"
		/>
	</div>
	<div class="w-full h-full text-accent font-mono text-xs">
		<Input index={strip_index} />
		<div>
			{#each Array(10) as _, index (index)}
				<Effect
					effect={strip.chain[index]}
					{index}
					strip={strip_index}
					empty={strip.chain[index] == null}
				/>
			{/each}
		</div>
		<Output index={strip_index} />
	</div>

	{#if showInputControls}
		<Frame title={strip.input.name} width={256} visible={true}>
			{#each strip.input.controls as control, index (index)}
				<div class="flex flex-row">
					<p class="w-1/2">{control.name}</p>
					<input
						type="range"
						min={control.min}
						max={control.max}
						step={control.step}
						bind:value={control.value}
						class="w-1/2"
					/>
				</div>
			{/each}
		</Frame>
	{/if}
</div>

<style>
	input[type="range"] {
		--value: 50%;
		appearance: slider-vertical;
		-webkit-appearance: slider-vertical;

		margin: 0.5rem;
		width: 10px;
		height: 18.5rem;
	}

	input[type="range"]::-webkit-slider-runnable-track {
		background-image: linear-gradient(
			to bottom,
			#222222 0%,
			#000000 calc(100% - var(--value)),
			green 100%
		);
	}
</style>
