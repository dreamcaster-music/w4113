<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { listen } from "@tauri-apps/api/event";
	import Frame from "./components/Frame.svelte";
	import "./settings.css";

	export let visible: boolean = false;

	let hostnames: string[] = [];
	let outputDevices: string[] = [];
	let inputDevices: string[] = [];
	let outputStreams: string[] = [];
	let inputStreams: string[] = [];

	let hostname: string = "";
	let outputDevice: string = "";
	let inputDevice: string = "";
	let outputStream: string = "";
	let inputStream: string = "";
	let outputBufferSize: number = 0;
	let inputBufferSize: number = 0;

	let thread: boolean = false;

	invoke("list_hosts").then((hosts) => {
		hostnames = hosts as string[];
		hostname = hostnames[0];
	});

	function updateHost(hostname: string) {
		invoke("set_host", { name: hostname }).then(() => {
			invoke("list_output_devices").then((devices) => {
				outputDevices = devices as string[];
				if (outputDevice === "") {
					outputDevice = outputDevices[0];
				}
			});
			invoke("list_input_devices").then((devices) => {
				inputDevices = devices as string[];
				if (inputDevice === "") {
					inputDevice = inputDevices[0];
				}
			});
		});
	}

	function updateOutputDevice(device: string) {
		invoke("set_output_device", { name: device }).then(() => {
			invoke("list_output_streams").then((streams) => {
				outputStreams = streams as string[];
				outputStream = outputStreams[0];
			});
		});
	}

	function updateInputDevice(device: string) {
		invoke("set_input_device", { name: device }).then(() => {
			invoke("list_input_streams").then((streams) => {
				inputStreams = streams as string[];
				inputStream = inputStreams[0];
			});
		});
	}

	$: {
		try {
			invoke("set_output_stream", { stream: outputStream });
			outputBufferSize = outputStream
				.split(" ")[2]
				.split("-")[1] as any as number;
		} catch (e) {
			outputBufferSize = 0;
		}
	}

	$: {
		try {
			invoke("set_input_stream", { stream: inputStream });
			inputBufferSize = inputStream
				.split(" ")[2]
				.split("-")[1] as any as number;
		} catch (e) {
			inputBufferSize = 0;
		}
	}

	$: updateHost(hostname);
	$: updateOutputDevice(outputDevice);
	$: updateInputDevice(inputDevice);

	const rustUpdateThread = listen("updatethread", (event) => {
		thread = event.payload as boolean;
	});
</script>

<Frame title="Audio Settings" width={700} {visible}>
	<div class="option">
		Host:
		<select bind:value={hostname}>
			{#each hostnames as hostname}
				<option value={hostname}>{hostname}</option>
			{/each}
		</select>
	</div>
	<div class="option">
		Output Device:
		<select bind:value={outputDevice}>
			{#each outputDevices as device}
				<option value={device}>{device}</option>
			{/each}
		</select>
	</div>

	<div class="option">
		Input Device:
		<select bind:value={inputDevice}>
			{#each inputDevices as device}
				<option value={device}>{device}</option>
			{/each}
		</select>
	</div>

	<div class="option">
		Output Stream:
		<select bind:value={outputStream}>
			{#each outputStreams as stream}
				<option value={stream}>{stream}</option>
			{/each}
		</select>
	</div>

	<div class="option">
		Input Stream:
		<select bind:value={inputStream}>
			{#each inputStreams as stream}
				<option value={stream}>{stream}</option>
			{/each}
		</select>
	</div>

	<div class="option w-auto">
		Output Buffer Size: <input value={outputBufferSize} class="w-10" />
		Input Buffer Size: <input value={inputBufferSize} class="w-10" />
	</div>

	<div class="option w-auto">
		Thread:
		<button
			class="w-full border-accent border-1"
			on:click={() => {
				if (!thread) {
					invoke("audio_thread");
				}
			}}>{thread ? "Running..." : "Start"}</button
		>
	</div>
</Frame>
