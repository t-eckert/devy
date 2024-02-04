import type { PageServerLoad } from './$types';
import api from '$lib/api';

export const load: PageServerLoad = async () => {
	const feeds = await Promise.all(
		['new', 'popular'].map(async (feed) => {
			const posts = await api.get('/v1/feeds/' + feed + '/posts');
			const config = await api.get('/v1/feeds/' + feed + '/config');

			return {
				feed,
				posts,
				config
			};
		})
	);

	return {
		props: {
			feeds
		}
	};
};
