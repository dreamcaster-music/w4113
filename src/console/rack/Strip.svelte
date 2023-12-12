<script lang="ts">
	import Output from "./Output.svelte";

	const width = 256;
	export let value = 50;
	export let index: number;
	export let input: string = "none";
	export let chain: string[] = [];
	export let output: string = "none";
</script>

<div
	class="h-full border-r-2 flex col-auto text-white font-mono"
	style="left: {index *
		width}px; width: {width}px; min-width: {width}px; border-color: gray;"
>
	<div class="flex flex-col">
		<p class="ml-3 mt-3">{index}</p>
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
			on:click={() => {}}
		>
			{input}
		</button>
		<div>
			{#each Array(10) as _, index (index)}
				<button
					class="ml-2 mr-2 w-10/12 h-6 border-1 border-accent-alt"
					on:click={() => {}}
				>
					{#if chain[index]}
						{chain[index]}
					{/if}
				</button>
			{/each}
		</div>
		<Output />
	</div>
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
