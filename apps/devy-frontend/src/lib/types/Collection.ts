import type Entry from "./Entry"

export interface Collection {
	config: CollectionConfig
	page: number
	pageSize: number
	count: number
	entries: Entry[]
}

export interface CollectionConfig {
	id: string
	name: string
}
