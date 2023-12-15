<script lang="ts">
	import { emit } from "@tauri-apps/api/event";

	enum InputType {
		"None",
		"Mono",
		"Stereo",
		"Bus",
		"Generator",
	}
	InputType;
	function typeFromString(type: string): InputType {
		switch (type) {
			case "Mono":
				return InputType.Mono;
			case "Stereo":
				return InputType.Stereo;
			case "Bus":
				return InputType.Bus;
			case "Generator":
				return InputType.Generator;
		}
		return InputType.None;
	}

	let typeAsString: string = "None";
	// name only applies to generators
	export let name: string = "";
	export let type: InputType = InputType.None;

	// left is multi-purpose
	// for mono inputs, it is the channel
	// for stereo inputs, it is the left channel
	// for bus inputs, it is the bus index
	export let left: number = 0;
	export let right: number = 0;
	export let index: number;

	$: type = typeFromString(typeAsString);

	let payload = {};

	$: {
		switch (type) {
			case InputType.None:
				payload = {
					index,
					kind: "input-mono",
					channel: left,
				};
				emit("svelte-updatestrip", payload);
				break;
			case InputType.Mono:
				break;
			case InputType.Stereo:
				payload = {
					index,
					kind: "input-stereo",
					left,
					right,
				};
				emit("svelte-updatestrip", payload);
				break;
			case InputType.Generator:
				payload = {
					index,
					kind: "input-generator",
					channel: left,
				};
			default:
				break;
		}
	}
</script>

<div class="m-2 w-10/12 h-6 flex flex-row">
	<select
		class="input-type font-mono w-full font-normal text-xs h-full border-1 border-accent"
		bind:value={typeAsString}
	>
		<option>Mono</option>
		<option>Stereo</option>
		<option>Generator</option>
		<option>Bus</option>
	</select>
	{#if type === InputType.Bus}
		<select
			class="font-mono font-normal text-xs h-full border-1 border-accent"
			bind:value={left}
		>
			{#each Array(64) as _, index (index)}
				<option>{index}</option>
			{/each}
		</select>
	{/if}
	{#if type === InputType.Generator}
		<select
			class="font-mono font-normal text-xs h-full w-full border-1 border-accent"
			bind:value={name}
		>
			<option>Sine</option>
			<option>Sampler</option>
		</select>
	{/if}
	{#if type === InputType.Mono}
		<select
			class="font-mono font-normal text-xs h-full border-1 border-accent"
			bind:value={left}
		>
			{#each Array(64) as _, index (index)}
				<option>{index}</option>
			{/each}
		</select>
	{/if}
	{#if type === InputType.Stereo}
		<select
			class="font-mono font-normal text-xs h-full border-1 border-accent"
			bind:value={left}
		>
			{#each Array(64) as _, index (index)}
				<option>{index}</option>
			{/each}
		</select>
		<select
			class="font-mono font-normal text-xs h-full border-1 border-accent"
			bind:value={right}
		>
			{#each Array(64) as _, index (index)}
				<option>{index}</option>
			{/each}
		</select>
	{/if}
</div>

<style>
	.input-type {
		min-width: 2rem;
	}

	select {
		min-width: 2rem;
	}
</style>
