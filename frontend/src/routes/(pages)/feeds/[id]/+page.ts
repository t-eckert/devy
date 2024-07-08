import type { PageLoad } from './$types';
import type { Feed } from '$lib/types';
import { type NumericRange, error } from '@sveltejs/kit';

export const load: PageLoad = async ({ fetch, params }) => {
	const resp = await fetch(`/api/feeds/${params.id}`);
	if (!resp.ok) {
		throw error(resp.status as NumericRange<400, 509>, resp.statusText);
	}

	const feed = (await resp.json()) as Feed;

	return {
		props: {
			feed
		}
	};
};
