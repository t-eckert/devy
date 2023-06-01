"use client"

import { FeedState } from "./useFeed"

import Context from "./Context"
import Tabs from "./Tabs"
import List from "./List"

export interface Props extends FeedState {}

const Feed: React.FC<Props> = (props) => {
	return (
		<Context {...props}>
			<section className="flex flex-col gap-4">
				<Tabs />
				<List />
			</section>
		</Context>
	)
}

export default Feed
