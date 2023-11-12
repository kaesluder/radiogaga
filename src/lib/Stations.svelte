<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Fa from 'svelte-fa';
	import { faRadio } from '@fortawesome/free-solid-svg-icons';

	type Station = {
		[key: string]: number | string;
	};

	let stations: Station[] = [];
	let searchString: string = '';
	let showPlayer = false;
	let selectedStation: null | Station = null;

	function activatePlayer(station: Station) {
		showPlayer = true;
		selectedStation = station;
	}

	async function getStations() {
		stations = await invoke('stations', { searchString });
	}

	getStations();
</script>

<div>
	<h2>Radio Stations</h2>
	<div class="flex flex-wrap">
		<div class="flex-auto">
			<input
				class="bg-stone-800"
				id="greet-input"
				placeholder="Enter a search string"
				bind:value={searchString}
			/>
		</div>
		<div class="flex-initial">
			<button class="bg-stone-150" on:click={getStations}>Search</button>
		</div>
	</div>

	{#if showPlayer && selectedStation}
		<div>
			<h3>{selectedStation['name']} : {selectedStation['codec']} : {selectedStation['bitrate']}</h3>
			<audio controls src={String(selectedStation['url'])} />
		</div>
	{/if}

	<div class="flex flex-wrap">
		{#each stations as station}
			<div
				class="hover:outline m-2 bg-stone-700 w-full sm:w-1/2 md:w-1/3 lg:w-1/4 xl:w-1/6 mb-4 p-4"
			>
				<h3 class="text-3xl">{station['name']}</h3>
				<div class="h-80">
					{#if station['favicon']}
						<img class="h-full object-contain" src={String(station['favicon'])} alt="Icon" />
					{:else}
						<Fa size="7x" icon={faRadio} />
					{/if}
				</div>

				<div>{station['codec']} {station['bitrate']}</div>
				<div>Clicks: {station['clickcount']}</div>
				<button on:click={activatePlayer(station)}>Play</button>
			</div>
		{/each}
	</div>
</div>
