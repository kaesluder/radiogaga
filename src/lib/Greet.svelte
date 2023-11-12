<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';
	let greetMsg = '';
	let logMsg = [];

	async function greet() {
		greetMsg = await invoke('greet', { name });
		logMsg = await invoke('stations');
		console.log(JSON.stringify(logMsg));
	}
</script>

<div class="flex flex-wrap">
	<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
	<button on:click={greet}>Greet</button>
	<p>{greetMsg} <img src="https://tauri.app/meta/tauri_logo_dark.svg"></p>
</div>
