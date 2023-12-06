<script lang="ts">
	import Frame from "./components/Frame.svelte";
	import "./settings.css";
</script>

<Frame title="Audio Settings" width={700}>
	<div class="option">
		Host:
		<select> </select>
	</div>
	<div class="option">
		Output Device:
		<select> </select>
	</div>

	<div class="option">
		Input Device:
		<select> </select>
	</div>

	<div class="option">
		Output Stream:
		<select> </select>
	</div>

	<div class="option">
		Input Stream:
		<select> </select>
	</div>

	<div class="option">
		Output Buffer Size: <input type="number" />
		Input Buffer Size: <input type="number" />
	</div>
</Frame>
