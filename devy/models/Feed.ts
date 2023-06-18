import db from "@/db"
import { Tag } from "@prisma/client"
import Post from "./Post"
import type Translator from "@/interfaces/Translator"

export default interface Feed {
	id: number
	slug: string
	name: string
	publishOrder: string
	filterTags: Tag[]
	lastCachedAt: Date
	posts: Post[]
}

export const feedTranslator: Translator<any, any> = {
	toModel: (prisma) => ({}),
	toPrisma: (model) => ({}),
}

export const feedCreator = {}

export const feedGetter = {
	default: async (): Promise<Feed[]> => {
		return []
	},
}
