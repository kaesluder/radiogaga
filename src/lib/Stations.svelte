<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	type Station = {
		[key: string]: number | String;
	};

	let stations: Station[] = [];
	let searchString: string = '';

	async function getStations() {
		stations = await invoke('stations', { searchString });
	}

	getStations();
</script>

<div>
	<h2>Radio Stations</h2>
	<input
		class="bg-stone-800"
		id="greet-input"
		placeholder="Enter a name..."
		bind:value={searchString}
	/>
	<button class="bg-stone-150" on:click={getStations}>Search</button>
	<div class="flex flex-wrap">
		{#each stations as station}
			<div class="w-full sm:w-1/2 md:w-1/3 lg:w-1/4 xl:w-1/6 mb-4 p-4">
				<h3 class="text-3xl">{station['name']}</h3>
				<div><img src={String(station['favicon'])} alt="Icon" /></div>
				<div>{station['codec']} {station['bitrate']}</div>
				<div>Clicks: {station['clickcount']}</div>
			</div>
		{/each}
	</div>
</div>
