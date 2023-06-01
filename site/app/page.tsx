import { getNewPostsMetadata, getPostsMetadataByPopularity } from "lib/post"

import Feed, { FeedState, useFeed } from "sections/Feed"
import Intro from "sections/Intro"

const HomePage = async () => {
	const feedState: FeedState = {
		currentFeed: "Popular",
		feeds: [
			{ name: "Popular", posts: await getPostsMetadataByPopularity() },
			{ name: "New", posts: await getNewPostsMetadata() },
		],
	}

	useFeed.setState(feedState)

	return (
		<div>
			<main className="px-2 max-w-3xl md:mt-12 mx-auto flex flex-col-reverse gap-12 md:flex-row md:gap-4">
				<div className="flex flex-col w-full max-w-2xl">
					<Feed {...feedState} />
				</div>
				<Intro />
			</main>
		</div>
	)
}

export default HomePage
