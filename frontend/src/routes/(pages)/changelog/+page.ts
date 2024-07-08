import type { PageLoad } from './$types';
import { error, type NumericRange } from '@sveltejs/kit';

export const load: PageLoad = async ({ fetch }) => {
	const resp = await fetch('/changelog/index.json', { headers: { accept: 'application/json' } });
	if (!resp.ok) {
		throw error(resp.status as NumericRange<400, 599>, {
			message: resp.statusText
		});
	}

	let index;
	try {
		index = await resp.json();
	} catch (e) {
		throw error(500, {
			message: e.message
		});
	}

	// t-eckert: this can be made faster with promise all settled
	const changelogs = [];
	for (const version of index) {
		const url = `/changelog/${version}.md`;
		console.log(url);
		const resp = await fetch(url, { headers: { accept: 'text/markdown' } });
		console.log(resp);
		if (!resp.ok) {
			throw error(resp.status as NumericRange<400, 599>, {
				message: resp.statusText
			});
		}

		let markdown;
		try {
			markdown = await resp.text();
		} catch (e) {
			throw error(500, {
				message: e.message
			});
		}

		changelogs.push({ version, markdown });
	}

	return {
		props: { changelogs }
	};
};
