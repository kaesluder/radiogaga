import axios from 'axios';

/**
 * Test if url points to a playlist or directly to a
 * stream file. Returns true if url points to a playlist.
 * Ignores query strings appended to the url.
 *
 * @param url stream or playlist url
 * @returns true if url points to a playlist
 */
export const isPlaylistUrl = function (url: string): boolean {
	return Boolean(url.split(/[?#]/)[0].match(/\.pls$/));
};

/**
 * Fetch a remotely hosted playlist and return the raw text.
 *
 * @param url url pointing to a playlist file
 * @returns string contents of playlist or null on failure
 */
export const fetchPlaylistData = async function (url: string): Promise<string | null> {
	try {
		const response = await axios.get(url);
		return response.data;
	} catch (error) {
		console.log(error);
		return null;
	}
};

/**
 * Parses playlist data and returns first specified url.
 * Returns null if parse fails.
 *
 * @param playlist string containing playlist data
 * @returns string url for stream or null
 */
export const parsePlaylist = function (playlist: string): string | null {
	const lines = playlist.split(/[\r\n]+/);

	// check that return result is a playlist
	if (!lines[0].match(/playlist/)) return null;
	for (const line of lines) {
		if (line.match(/^File/)) return line.split('=')[1];
	}
	return null;
};
