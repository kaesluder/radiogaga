import { describe, expect, test, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import Stations from './Stations.svelte';
import '@testing-library/jest-dom';
import { mockIPC } from '@tauri-apps/api/mocks';

type Station = {
	[key: string]: number | string;
};

const testStation: Station = { name: 'foo', codec: 'bar', bitrate: 23 };

describe('Stations', () => {
	beforeEach(() => {
		mockIPC((cmd) => {
			if (cmd == 'stations') return [testStation];
		});
		//create instance of the component and mount it
	});

	afterEach(() => {
		// run after each
	});

	test('display button', () => {
		render(Stations, {});

		expect(screen.getByRole('button')).toBeInTheDocument();
	});

	test('display test station', async () => {
		await render(Stations, {});

		await fireEvent(
			screen.getByRole('button'),
			new MouseEvent('click', {
				bubbles: true,
				cancelable: true
			})
		);

		await waitFor(() => expect(screen.getByText(/23/)).toBeInTheDocument());
		await waitFor(() => expect(screen.getByText(/foo/)).toBeInTheDocument());
		await waitFor(() => expect(screen.getByText(/bar/)).toBeInTheDocument());
	});
});
