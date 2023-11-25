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

	/**
	 * Activates the player by setting the `showPlayer` flag to true, assigning the
	 * selected station to the `selectedStation` variable, and fetching the stream URL
	 * for the selected station using the `checkStreamUrl` function.
	 *
	 * @param {Station} station - The selected station object.
	 * @returns {Promise<void>} - A promise that resolves when the stream URL is fetched and logged to the console.
	 */
	async function activatePlayer(station: Station) {
		showPlayer = true;
		selectedStation = station;
		streamUrl = await checkStreamUrl(String(selectedStation['url']));
		console.log(streamUrl);
	}

	function formatVoteNum(num: number | string): string {
		if (typeof num !== 'number') return '0';
		if (num > 1000) return `${Math.floor(num / 1000)}K`;
		if (num > 1000000) return `${Math.floor(num / 1000000)}M`;
		return num.toString();
	}

	/**
	 * Function to handle key press events.
	 *
	 * @param {KeyboardEvent} event - The event object representing the key press event.
	 * @returns {void} - This function does not return any value.
	 */
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

	/**
	 * Function to deactivate the player.
	 *
	 * This function is responsible for deactivating the player by setting the `showPlayer` flag to false,
	 * and resetting the `selectedStation` variable to null.
	 *
	 * @returns {void}
	 */
	function deactivatePlayer() {
		console.log('closing player');
		showPlayer = false;
		selectedStation = null;
	}
	/**
	 * Asynchronous function that retrieves a list of stations.
	 *
	 * @returns {Promise<void>} A promise that resolves when the stations are retrieved.
	 */
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
				class="rounded-md hover:outline p-2 m-2 bg-stone-700 w-[200px] h-[350px] flex flex-col justify-between"
			>
				<h3 class="text-3xl">{station['name']}</h3>
				<div>
					{#if station['favicon']}
						<img class="w-full object-contain" src={String(station['favicon'])} alt="Icon" />
					{:else}
						<Fa size="7x" icon={faRadio} />
					{/if}
				</div>

				<div class="mt-auto">{station['codec']} {station['bitrate']}</div>
				<div>
					Votes: {formatVoteNum(station['votes'])}
				</div>
				<button
					on:click={() => {
						deactivatePlayer();
						activatePlayer(station);
					}}
				>
					Play
				</button>
			</div>
		{/each}
	</div>
</div>
