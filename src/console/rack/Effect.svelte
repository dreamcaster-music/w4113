<script lang="ts">
	import { emit } from "@tauri-apps/api/event";
	import ContextMenu from "../components/ContextMenu.svelte";
	import Frame from "../components/Frame.svelte";

	enum EffectOptions {
		BitCrusher = "BitCrusher",
		Delay = "Delay",
		Remove = "Remove",
	}

	export let strip: number;
	export let index: number;
	export let effect: any;
	export let empty: boolean = false;

	let showControls: boolean = false;
	let contextMenu: { x: number; y: number; options: string[] } = {
		x: 0,
		y: 0,
		options: [],
	};

	let contextMenuCallback = (option: string) => {
		switch (option) {
			case EffectOptions.Delay:
				emit("svelte-seteffect", { option, strip, index });
				break;
			case EffectOptions.BitCrusher:
				emit("svelte-seteffect", { option, strip, index });
				break;
			case EffectOptions.Remove:
				emit("svelte-removeeffect", { strip, index });
				break;
		}
	};
</script>

{#if contextMenu.options.length > 0}
	<ContextMenu
		callback={contextMenuCallback}
		options={contextMenu.options}
		x={contextMenu.x}
		y={contextMenu.y}
	/>
{/if}

{#if empty}
	<button
		class="ml-2 mr-2 w-10/12 h-6 border-1 border-accent-alt"
		on:click={() => {
			showControls = !showControls;
		}}
		on:contextmenu={(event) => {
			setTimeout(() => {
				contextMenu = {
					y: event.clientY,
					x: event.clientX,
					options: [EffectOptions.BitCrusher, EffectOptions.Delay],
				};
			}, 0);
		}}
	>
		empty
	</button>
{/if}

{#if !empty}
	<button
		class="ml-2 mr-2 w-10/12 h-6 border-1 border-accent-alt"
		on:click={() => {
			showControls = !showControls;
		}}
		on:contextmenu={(event) => {
			setTimeout(() => {
				contextMenu = {
					y: event.clientY,
					x: event.clientX,
					options: [
						EffectOptions.BitCrusher,
						EffectOptions.Delay,
						EffectOptions.Remove,
					],
				};
			}, 0);
		}}
	>
		{effect.name}
	</button>
{/if}

{#if showControls && !empty}
	<Frame title={effect.name} width={256} visible={true}>
		{#each effect.controls as control}
			{#if control.kind == "dial"}
				<div class="flex flex-row">
					<p class="w-1/2">{control.name}</p>
					<p class="w-1/2 text-right">{control.min}</p>
					<input
						type="range"
						min={control.min}
						max={control.max}
						bind:value={control.value}
						on:input={() => {
							emit("svelte-updatestrip", {
								kind: "control",
								control: control.kind,
								strip,
								index,
								name: control.name,
								value: control.value,
							});
						}}
						class="w-1/2"
					/>
					<p class="w-1/2">{control.max}</p>
				</div>
			{/if}
			{#if control.kind == "slider"}
				<div class="flex flex-row">
					<p class="w-1/2">{control.name}</p>
					<p class="w-1/2 text-right">{control.min}</p>
					<input
						type="range"
						min={control.min}
						max={control.max}
						bind:value={control.value}
						on:input={() => {
							emit("svelte-updatestrip", {
								kind: "control",
								control: control.kind,
								strip,
								index,
								name: control.name,
								value: control.value,
							});
						}}
						class="w-1/2"
					/>
					<p class="w-1/2">{control.max}</p>
				</div>
			{/if}
			{#if control.kind == "toggle"}
				<div class="flex flex-row">
					<p class="w-1/2">{control.name}</p>
					<button
						class="w-1/2"
						on:click={() => {
							control.value =
								control.value + (1 % control.n_states);
						}}
						on:change={() => {
							emit("svelte-updatestrip", {
								kind: "control",
								control: control.kind,
								strip,
								index,
								name: control.name,
								value: control.value,
							});
						}}
					>
						{control.value}
					</button>
				</div>
			{/if}
		{/each}
	</Frame>
{/if}
