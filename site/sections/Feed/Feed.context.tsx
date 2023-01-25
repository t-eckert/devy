"use client"

import { useEffect, useState } from "react"

import Feed from "./Feed"

import { defaults } from "lib/feed"
import { PostMetadata, getPostsMetadataByFeed } from "lib/post"

const FeedContext: React.FC = () => {
  const feeds = defaults
  const [posts, setPosts] = useState<PostMetadata[]>([])
  const [feedId, setFeedId] = useState<string>(defaults[0].id)
  const [pageNumber, setPageNumber] = useState<number>(1)

  useEffect(() => {
    getPostsMetadataByFeed(feedId).then((posts) => {
      setPosts(posts)
    })
  }, [feedId, pageNumber])

  return (
    <Feed
      postsMetadata={posts}
      feeds={feeds}
      feedId={feedId}
      setFeedId={setFeedId}
      pageNumber={pageNumber}
      setPageNumber={setPageNumber}
    />
  )
}

export default FeedContext
