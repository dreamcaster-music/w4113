<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { open } from "@tauri-apps/api/dialog";
	import { error } from "tauri-plugin-log-api";
	import Frame from "./components/Frame.svelte";

	export let visible: boolean = false;
	let customSample: string[] = [];
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

	<button
		class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
		on:click={() => {
			error("I have been clicked");
			open({
				multiple: true,
				filters: [
					{
						name: "Audio Files",
						extensions: ["mp3", "wav"],
					},
				],
			}).then((result) => {});
		}}
	>
		+ Custom Sample
	</button>
</Frame>
