<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { debug } from "tauri-plugin-log-api";
	import Frame from "./components/Frame.svelte";

	export let visible: boolean = false;
	let list: string[] = [];

	invoke("list_interfaces").then((result) => {
		debug("list_interfaces: " + result);
		let id = 966156933;
		list = result as string[];
	});
</script>

<Frame title="Keyboard Settings" width={700} {visible}>
	<p class="text-accent text-center">
		Select a interface to use as a keyboard.
	</p>
	<p class="text-accent text-center">
		{#if list.length === 0}
			No interfaces found.
		{:else}
			{#each list as item}
				<div class="flex justify-center">
					<p
						class="w-full text-accent border-1 border-accent p-1 m-2 select-text text-left"
					>
						{item}
					</p>
				</div>
			{/each}
		{/if}
	</p>
</Frame>
