<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { open } from "@tauri-apps/api/dialog";
	import { info } from "tauri-plugin-log-api";
	import Frame from "./components/Frame.svelte";

	export let visible: boolean = false;
	let customSamples: string[] = [];

	function addSample(sample: string) {
		customSamples.push(sample);
	}
</script>

<Frame title="Reaver Super Sampler 7" width={700} {visible}>
	<button
		class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
		on:click={() => {
			invoke("play_sample", { path: "./assets/reaved.mp3" });
		}}
	>
		reaved.mp3
	</button>
	<button
		class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
		on:click={() => {
			invoke("play_sample", { path: "./assets/pinkfloyd.mp3" });
		}}
	>
		pinkfloyd.mp3
	</button>
	<button
		class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
		on:click={() => {
			invoke("play_sample", { path: "./assets/oof.mp3" });
		}}
	>
		oof.mp3
	</button>
	<button
		class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
		on:click={() => {
			invoke("play_sample", { path: "./assets/ahh.mp3" });
		}}
	>
		ahh.mp3
	</button>
	<button
		class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
		on:click={() => {
			invoke("play_sample", { path: "./assets/pipe.mp3" });
		}}
	>
		pipe.mp3
	</button>
	<button
		class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
		on:click={() => {
			invoke("play_sample", { path: "./assets/5fznfr.wav" });
		}}
	>
		5fznfr.wav
	</button>

	{#each customSamples as sample}
		<button
			class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
			on:click={() => {
				invoke("play_sample", { path: sample });
			}}
		>
			{sample}
		</button>
	{/each}

	<button
		class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
		on:click={() => {
			open({
				multiple: false,
				filters: [
					{
						name: "Audio Files",
						extensions: ["mp3", "wav"],
					},
				],
			}).then((result) => {
				info("" + result);
				//add result to customSamples array
				if (result != null) {
					let resultString = result.toString();
					info("result[0]: " + result);
					addSample(resultString);
					info("customSamples: " + customSamples);
				}
			});
		}}
	>
		+ Custom Sample
	</button>
</Frame>
