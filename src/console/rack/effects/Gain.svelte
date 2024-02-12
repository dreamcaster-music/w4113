<script lang="ts">
	let size = 40;
	let value = 10;

	let listenMouse = false;
	let mouseLastY = 0;

	let min = 45;
	let max = 315;

	function radians(degrees: number) {
		let pi = Math.PI;
		return degrees * (pi / 180);
	}

	function degrees(radians: number) {
		let pi = Math.PI;
		return radians * (180 / pi);
	}

	function translate(value: number) {
		// 0 = 315
		// 50 = 225
		/// 100 = 45

		let result = (100 - value) * 2.7 + 45;

		return result;
	}
</script>

<button
	class="background"
	on:mousedown={(e) => {
		mouseLastY = e.clientY;
		listenMouse = true;
	}}
	on:mousemove={(e) => {
		if (listenMouse) {
			let y = e.clientY;
			let diff = y - mouseLastY;

			value -= diff;

			if (value > 100) {
				value = 100;
			}

			if (value < 0) {
				value = 0;
			}

			mouseLastY = y;
		}
	}}
>
	<p class="text-center absolute text-white text-3xl w-full h-full">
		{value}
	</p>

	<!-- Draw semicircle from left to right over top -->
	<svg
		width="100%"
		height="100%"
		viewBox="0 0 100 100"
		xmlns="http://www.w3.org/2000/svg"
	>
		<path
			d="M {Math.sin(radians(max)) * size + 50} {Math.cos(radians(max)) *
				size +
				50}
				A {size} {size} 0 1 1 {Math.sin(radians(min)) * size + 50} {Math.cos(
				radians(min),
			) *
				size +
				50}"
			fill="none"
			stroke="gray"
			stroke-width="10"
		/>

		<path
			d="M {Math.sin(radians(max)) * size + 50} {Math.cos(radians(max)) *
				size +
				50}
				A {size} {size} 0 {translate(value) >= 135 ? 0 : 1} 1 {Math.sin(
				radians(translate(value)),
			) *
				size +
				50} {Math.cos(radians(translate(value))) * size + 50}"
			fill="none"
			stroke="lime"
			stroke-width="10"
		/>
	</svg>
</button>

<svelte:window on:mouseup={() => (listenMouse = false)} />

<style>
	p {
		top: 50%;
	}
</style>
