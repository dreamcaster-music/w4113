<script lang="ts">
	export let options: string[];
	export let x: number;
	export let y: number;
	export let callback: (option: any) => void;
</script>

<div class="flex flex-col fixed bg-black" style="left: {x}px; top: {y}px;">
	{#each options as option, index (index)}
		<button
			class="w-auto h-6 font-mono text-xs border-1 border-accent text-accent nowrap pl-2 pr-2"
			on:click={() => {
				callback(option);
			}}
		>
			{option}
		</button>
	{/each}
</div>

<svelte:window
	on:contextmenu={(e) => {
		e.preventDefault();
		options = [];
	}}
	on:click={(e) => {
		options = [];
	}}
/>

<style>
	.nowrap {
		white-space: nowrap;
	}
</style>
