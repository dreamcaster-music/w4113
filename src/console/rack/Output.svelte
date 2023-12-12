<script lang="ts">
	enum OutputType {
		"None",
		"Mono",
		"Stereo",
		"Bus",
	}

	function typeFromString(type: string): OutputType {
		switch (type) {
			case "Mono":
				return OutputType.Mono;
			case "Stereo":
				return OutputType.Stereo;
			case "Bus":
				return OutputType.Bus;
		}
		return OutputType.None;
	}

	let typeAsString: string = "None";
	export let type: OutputType = OutputType.None;

	$: type = typeFromString(typeAsString);
</script>

<div class="m-2 w-10/12 h-6 flex flex-row">
	<select
		class="input-type font-mono w-full font-normal text-xs h-full border-1 border-accent"
		bind:value={typeAsString}
	>
		<option>None</option>
		<option>Mono</option>
		<option>Stereo</option>
		<option>Bus</option>
	</select>
	{#if type === OutputType.Bus}
		<input
			type="text"
			class="font-mono font-normal text-xs h-full border-1 border-accent"
			placeholder="Bus Name"
		/>
	{/if}
	{#if type === OutputType.Mono}
		<select
			class="font-mono font-normal text-xs h-full border-1 border-accent"
		>
			{#each Array(64) as _, index (index)}
				<option>{index}</option>
			{/each}
		</select>
	{/if}
	{#if type === OutputType.Stereo}
		<select
			class="font-mono font-normal text-xs h-full border-1 border-accent"
		>
			{#each Array(64) as _, index (index)}
				<option>{index}</option>
			{/each}
		</select>
		<select
			class="font-mono font-normal text-xs h-full border-1 border-accent"
		>
			{#each Array(64) as _, index (index)}
				<option>{index}</option>
			{/each}
		</select>
	{/if}
</div>

<style>
	.input-type {
		min-width: 4rem;
	}

	select {
		min-width: 2rem;
	}
</style>
