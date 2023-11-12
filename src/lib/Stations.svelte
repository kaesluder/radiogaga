<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	type Station = {
		[key: string]: number | String;
	};

	let stations: Station[] = [];

	async function getStations() {
		stations = await invoke('stations');
	}

	getStations();
</script>

<div>
	<h2>Radio Stations</h2>
	<div class="flex flex-wrap">
		{#each stations as station}
			<div class="w-full sm:w-1/2 md:w-1/3 lg:w-1/4 xl:w-1/6 mb-4 p-4">
				<div class="max-w-sm rounded overflow-hidden bg-gray-800">
					<h3>{station['name']}</h3>
					<div><img src={String(station['favicon'])} alt="Icon" /></div>
					<div>{station['codec']} {station['bitrate']}</div>
				</div>
			</div>
		{/each}
	</div>
</div>
