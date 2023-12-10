<script lang="ts">
	import Frame from "./components/Frame.svelte";

	export let visible: boolean = false;
</script>

<Frame title="Keyboard Settings" width={700} {visible}></Frame>
