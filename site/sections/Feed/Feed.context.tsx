import { useState } from "react"

import Feed from "./Feed"
import { defaults } from "lib/feed"
import { getPostsMetadataByFeed } from "lib/post"

const FeedContext: React.FC = () => {
  // TODO this will be where a user's feeds are fetched.
  const feeds = defaults

  const [feedId, setFeedId] = useState<string>(defaults[0].id)
  const [pageNumber, setPageNumber] = useState<number>(1)

  return (
    <Feed
      postsMetadata={getPostsMetadataByFeed(feedId)}
      feeds={feeds}
      feedId={feedId}
      setFeedId={setFeedId}
      pageNumber={pageNumber}
      setPageNumber={setPageNumber}
    />
  )
}

export default FeedContext
