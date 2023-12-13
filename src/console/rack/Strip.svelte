<script lang="ts">
	import Frame from "../components/Frame.svelte";
	import Effect from "./Effect.svelte";
	import Output from "./Output.svelte";

	const width = 256;
	export let value = 50;
	export let strip_index: number;
	export let strip: any;

	let showInputControls = false;
</script>

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
		<button
			class="w-10/12 h-6 border-1 border-accent m-2"
			on:click={() => {
				showInputControls = !showInputControls;
			}}
		>
			{strip.input.name}
		</button>
		<div>
			{#each Array(10) as _, index (index)}
				{#if strip.chain[index]}
					<Effect effect={strip.chain[index]} strip={strip_index} />
				{/if}
				{#if !strip.chain[index]}
					<button
						class="ml-2 mr-2 w-10/12 h-6 border-1 border-accent-alt"
					>
						empty
					</button>
				{/if}
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
