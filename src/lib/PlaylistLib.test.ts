import { describe, expect, test, vi, beforeEach, afterEach } from 'vitest';
import axios from 'axios';
import { isPlaylistUrl, fetchPlaylistData, parsePlaylist } from './PlaylistLib';

const playlistURL: string = 'http://www.example.com/playlist.pls';
const mp3Url: string = 'http://www.example.com/playlist.mp3';

const playlistBody: string = `[playlist]
File1=http://example.com/example.mp3`;

// playlist text lacking valid header line
const badPlaylistBody1 = `[init]
File1=http://example.com/example.mp3`;

// playlist text lacking valid `FileX=` line entry.
const badPlaylistBody2 = `[playlist]
Title1=Total Eclipse of The Heart`;

vi.mock('axios');

describe('PlaylistLib', () => {
	test('url ending with mp3 returns false', () => {
		expect(isPlaylistUrl(mp3Url)).toBe(false);
	});

	test('url ending with pls returns true', () => {
		expect(isPlaylistUrl(playlistURL)).toBe(true);
	});

	test('url ending with pls?query returns true', () => {
		expect(isPlaylistUrl(playlistURL + '?query')).toBe(true);
	});

	test('url ending with mp3?query returns false', () => {
		expect(isPlaylistUrl(mp3Url + '?query')).toBe(false);
	});

	test('api call against mocked axios returns correct value', async () => {
		vi.mocked(axios.get).mockResolvedValue({ data: playlistBody });

		const result = await fetchPlaylistData('https://somafm.com/synphaera256.pls');
		expect(result).toBe(playlistBody);
	});

	test('playlist parser returns link to mp3 stream', () => {
		const result = parsePlaylist(playlistBody);
		expect(result).toBe('http://example.com/example.mp3');
	});

	test('playlist parser returns undefined on bad playlist header', () => {
		const result = parsePlaylist(badPlaylistBody1);
		expect(result).toBeUndefined();
	});

	test('playlist parser returns undefined if no file entry is found', () => {
		const result = parsePlaylist(badPlaylistBody2);
		expect(result).toBeUndefined();
	});
});
