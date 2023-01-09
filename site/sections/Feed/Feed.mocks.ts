import { Props } from "./Feed"
import fixtures from "fixtures/post"

const popularFeed: Props = {
  postsMetadata: fixtures.postsMetadata,
  feeds: [
    { id: "37e24b45-e5d2-456e-94ef-11f2c64ef17f", name: "Popular" },
    { id: "aee0f9e9-a028-4c8b-a453-dbdbcf309fc0", name: "New" },
  ],
  feedId: "37e24b45-e5d2-456e-94ef-11f2c64ef17f",
  setFeedId: (_: string) => { },
  pageNumber: 1,
  setPageNumber: (_: number) => { },
}

const mocks = {
  popularFeed,
}

export default mocks
