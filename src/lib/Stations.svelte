<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Fa from 'svelte-fa';
	import { faRadio, faClose } from '@fortawesome/free-solid-svg-icons';
	import { isPlaylistUrl, fetchPlaylistData, parsePlaylist } from './PlaylistLib';

	type Station = {
		[key: string]: number | string;
	};

	let stations: Station[] = [];
	let searchString: string = '';
	let showPlayer = false;
	let selectedStation: null | Station = null;
	let streamUrl: string | undefined = undefined;

	async function activatePlayer(station: Station) {
		showPlayer = true;
		selectedStation = station;
		streamUrl = await checkStreamUrl(String(selectedStation['url']));
		console.log(streamUrl);
	}

	function handleKeyPress(event: KeyboardEvent) {
		// Check if the Enter key is pressed (key code 13)
		if (event.key === 'Enter') {
			getStations();
		}
	}
	/**
	 * Validates and normalizes a stream URL.
	 *
	 * This function checks if the provided URL is a valid stream URL or a playlist URL.
	 * If it's a playlist URL, it will try to fetch and parse the playlist data to
	 * extract the actual stream URL. Otherwise, the original URL is returned.
	 *
	 * @param {string} url - The URL to be validated and normalized.
	 * @return {Promise<string | undefined>} A promise that resolves to a valid stream
	 *     URL or undefined if the input URL is not a valid stream URL.
	 */
	async function checkStreamUrl(url: string): Promise<string | undefined> {
		// TODO Refactor
		if (isPlaylistUrl(url)) {
			const playlistBody = await fetchPlaylistData(url);
			if (playlistBody) {
				return parsePlaylist(playlistBody);
			} else {
				return undefined;
			}
		} else {
			return url;
		}
	}

	function deactivatePlayer() {
		console.log('closing player');
		showPlayer = false;
		selectedStation = null;
	}

	async function getStations() {
		stations = await invoke('stations', { searchString });
	}

	getStations();
</script>

<div>
	<h2>Radio Stations</h2>
	<div class="flex flex-wrap m-3">
		<div class="flex-auto">
			<input
				class="bg-stone-800"
				id="greet-input"
				placeholder="Enter a search string"
				bind:value={searchString}
				on:keydown={handleKeyPress}
			/>
		</div>
		<div class="flex-initial">
			<button class="bg-stone-150" on:click={getStations}>Search</button>
		</div>
	</div>

	{#if showPlayer && selectedStation}
		<div class="bg-stone-800 fixed bottom-0 p-4 rounded">
			<h3>{selectedStation['name']} : {selectedStation['codec']} : {selectedStation['bitrate']}</h3>
			<div class="flex">
				<audio controls autoplay src={String(streamUrl)} />
				<button on:click={deactivatePlayer}><Fa icon={faClose} /> </button>
			</div>
		</div>
	{/if}

	<div class="flex flex-wrap justify-center items-center">
		{#each stations as station}
			<div
				class="rounded-md hover:outline m-2 bg-stone-700 w-full sm:w-1/2 md:w-1/3 lg:w-1/4 xl:w-1/6 mb-4 p-4"
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
				<button
					on:click={() => {
						deactivatePlayer();
						activatePlayer(station);
					}}>Play</button
				>
			</div>
		{/each}
	</div>
</div>
