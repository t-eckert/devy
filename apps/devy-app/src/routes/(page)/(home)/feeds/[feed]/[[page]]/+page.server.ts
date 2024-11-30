import type { PageServerLoad } from "./$types";
import { getFeed } from "$lib/server/feeds";

export const load: PageServerLoad = async ({ params }) => {
  const { feed, page } = params;

  return {
    feed: await getFeed(feed, Number(page || 1)),
  };
}
