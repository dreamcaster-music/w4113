<script lang="ts">
	import { emit } from "@tauri-apps/api/event";
	import Frame from "../components/Frame.svelte";

	export let strip: number;
	export let effect: any;

	let showControls: boolean = false;
</script>

<button
	class="ml-2 mr-2 w-10/12 h-6 border-1 border-accent-alt"
	on:click={() => {
		showControls = !showControls;
	}}
>
	{effect.name}
</button>

{#if showControls}
	<Frame title={effect.name} width={256} visible={true}>
		{#each effect.controls as control, index (index)}
			{#if control.kind == "dial"}
				<div class="flex flex-row">
					<p class="w-1/2">{control.name}</p>
					<p class="w-1/2 text-right">{control.min}</p>
					<input
						type="range"
						min={control.min}
						max={control.max}
						bind:value={control.value}
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
						on:mousemove={() => {
							emit("svelte-updatestrip", {
								kind: "control",
								control: control.kind,
								strip,
								index,
								name: control.name,
								value: control.value,
							});
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
