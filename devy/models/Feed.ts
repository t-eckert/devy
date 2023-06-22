import db from "@/db"
import { Tag } from "@prisma/client"
import Post from "./Post"
import type Translator from "@/interfaces/Translator"

import { postTranslator } from "./Post"

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
	toModel: (prisma) => ({
		id: prisma.id,
		slug: prisma.slug,
		name: prisma.name,
		publishOrder: prisma.publishOrder,
		filterTags: prisma.filterTags,
		lastCachedAt: new Date(),
		posts: prisma.posts.map((post: any) =>
			postTranslator.toModel(post.post)
		),
	}),
	toPrisma: (model) => ({}),
}

export const feedCreator = {}

export const feedGetter = {
	defaults: async (): Promise<Feed[]> => {
		const feeds = await db.feed.findMany({
			where: {
				slug: {
					in: ["new", "popular"],
				},
			},
			include: {
				posts: {
					include: {
						post: {
							include: {
								tags: true,
								blog: true,
								author: true,
								likes: true,
							},
						},
					},
				},
			},
		})
		return feeds.map((feed) => feedTranslator.toModel(feed))
	},
}
