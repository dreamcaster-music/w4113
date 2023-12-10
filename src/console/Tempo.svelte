<script lang="ts">
	let tempo: number = 120;
	let timeSignature: { numerator: number; denominator: number } = {
		numerator: 4,
		denominator: 4,
	};
</script>

<div
	class="border-1 border-accent text-accent w-auto h-auto absolute top-10 right-10 z-50 bg-black"
>
	<input
		class="text-accent w-8 border-none font-normal text-right"
		bind:value={tempo}
		on:input={() => {
			if (isNaN(tempo)) {
				tempo = 120;
			} else {
				if (tempo > 999) {
					// truncate last digit
					tempo = Math.floor(tempo / 10);
				}
			}
		}}
	/>
	<input
		class="text-accent w-5 border-none text-right font-normal"
		bind:value={timeSignature.numerator}
		on:input={() => {
			if (isNaN(timeSignature.numerator)) {
				timeSignature.numerator = 4;
			}
		}}
	/>
	/
	<input
		class="text-accent w-5 border-none text-left font-normal"
		bind:value={timeSignature.denominator}
		on:input={() => {
			if (isNaN(timeSignature.denominator)) {
				timeSignature.denominator = 4;
			}
		}}
	/>
</div>
