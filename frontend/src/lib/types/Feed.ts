import type Entry from "./Entry"
import FeedConfig from "./FeedConfig"

interface Feed {
	feedConfig: FeedConfig
	entries: Entry[]
}

export default Feed
