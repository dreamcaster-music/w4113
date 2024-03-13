<script lang="ts">
	import { emit } from "@tauri-apps/api/event";

	export let strip: number;
	export let index: number;

	let min = -12.0;
	let max = 24.0;
	let value = 0.0;

	let size = 30;

	let listenMouse = false;
	let mouseLastY = 0;

	let valueR = valueToRadius(value);
	let minR = 45;
	let maxR = 315;

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

	$: {
		value = radiusToValue(valueR);

		emit("svelte-seteffectvalue", {
			strip,
			index,
			value_name: "gain",
			value_kind: "float",
			value,
		});
	}

	function radiusToValue(value: number): number {
		let result = (value / 1000) * (max - min) + min;
		return result;
	}

	function valueToRadius(value: number): number {
		let result = ((value - min) / (max - min)) * 1000;
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
			let multiplier = 5;

			valueR -= diff * multiplier;

			if (valueR > 1000) {
				valueR = 1000;
			}

			if (valueR < 0) {
				valueR = 0;
			}

			mouseLastY = y;
		}
	}}
>
	<p class="text-center absolute text-white text-3xl w-full h-full">
		{value.toFixed(1)}
	</p>

	<!-- Draw semicircle from left to right over top -->
	<svg
		width="100%"
		height="100%"
		viewBox="0 0 100 100"
		xmlns="http://www.w3.org/2000/svg"
	>
		<path
			d="M {Math.sin(radians(maxR)) * size + 50} {Math.cos(
				radians(maxR),
			) *
				size +
				50}
				A {size} {size} 0 1 1 {Math.sin(radians(minR)) * size + 50} {Math.cos(
				radians(minR),
			) *
				size +
				50}"
			fill="none"
			stroke="gray"
			stroke-width="10"
		/>

		<path
			d="M {Math.sin(radians(maxR)) * size + 50} {Math.cos(
				radians(maxR),
			) *
				size +
				50}
				A {size} {size} 0 {translate(valueR / 10) >= 135 ? 0 : 1} 1 {Math.sin(
				radians(translate(valueR / 10)),
			) *
				size +
				50} {Math.cos(radians(translate(valueR / 10))) * size + 50}"
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
