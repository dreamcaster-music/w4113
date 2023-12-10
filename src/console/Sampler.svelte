<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import Frame from "./components/Frame.svelte";

	export let visible: boolean = false;
</script>

<Frame title="Reaver Super Sampler 7" width={700} {visible}>
	<button
		class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
		on:click={() => {
			invoke("play_sample", { path: "./public/reaved.mp3" });
		}}
	>
		reaved.mp3
	</button>
</Frame>
