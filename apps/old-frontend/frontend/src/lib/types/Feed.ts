import type Entry from "./Entry"
import type FeedConfig from "./FeedConfig"

export default interface Feed {
  config: FeedConfig
  page: number
  pageSize: number
  count: number
  entries: Entry[]
}
