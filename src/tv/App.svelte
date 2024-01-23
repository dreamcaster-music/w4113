<script lang="ts">
	import { listen } from "@tauri-apps/api/event";

	let data: any[] = [];

	listen("loadbars", (payload) => {
		// @ts-ignore
		data = payload.payload;
	});

	function style(value: number, i: number) {
		let output = `height: ${Math.abs(value * 300)}px;`;

		// color spectrum based on i
		let hue = (i * 100) / data.length;

		output += `background-color: hsl(${hue}, 70%, 70%);`;

		let positive = value > 0;

		if (positive) {
			output += "bottom: 50%;";
		} else {
			output += "top: 50%;";
		}

		output += `left: ${i * 2}px;`;

		return output;
	}
</script>

<div class="background" data-tauri-drag-region>
	{#each data as bar, i}
		<div
			class="absolute bottom-0 left-0 w-1 bg-white"
			style={style(bar, i)}
		></div>
	{/each}
</div>

<style>
	.background {
		margin: 0;
		padding: 0;
		position: absolute;
		width: 100%;
		height: 100%;
	}
</style>
