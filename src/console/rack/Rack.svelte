<script lang="ts">
	import { listen } from "@tauri-apps/api/event";
	import Strip from "./Strip.svelte";

	let strips: any[] = [];

	let updatestrip = listen("rust-updatestrip", (strip) => {
		// @ts-ignore
		let input = strip.payload.input;
		// @ts-ignore
		let chain = strip.payload.chain;
		// @ts-ignore
		let output = strip.payload.output;

		strips.push({ input, chain, output });
		strips = strips;
	});

	let removestrip = listen("rust-removestrip", (strip) => {
		// @ts-ignore
		let index = strip.payload as number;

		strips.splice(index, 1);
		strips = strips;
	});

	let clearstrips = listen("rust-clearstrips", () => {
		strips = [];
		strips = strips;
	});
</script>

<div
	class="absolute w-full h-auto bottom-0 border-t-2 border-gray-500 flex col-auto"
>
	{#each strips as strip, index}
		<Strip
			{index}
			input={strip.input}
			chain={strip.chain}
			output={strip.output}
		/>
	{/each}

	<!-- Add Strip -->
	<button
		class="text-white w-8 m-2 border-1 border-accent"
		on:click={() => {
			strips.push("");
			strips = strips;
		}}
	>
		+
	</button>
</div>

<style>
	div {
		min-height: 21rem;
		overflow-y: scroll;
	}

	.w-8 {
		min-width: 2rem;
	}
</style>
