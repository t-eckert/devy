import type Entry from "./Entry"

export default interface Feed {
  config: FeedConfig
  page: number
  pageSize: number
  count: number
  entries: Entry[]
}

export interface FeedConfig {
  id: string
  name: string
}
