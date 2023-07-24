import db from "@/db"
import {
	Tag as PrismaTag,
	Feed as PrismaFeed,
	Post as PrismaPost,
	Blog as PrismaBlog,
	User as PrismaUser,
	Order,
} from "@prisma/client"
import Post from "./Post"
import type Translator from "@/interfaces/Translator"

import { postTranslator } from "./Post"

export default interface Feed {
	id: number
	slug: string
	name: string
	publishOrder: string
	filterTags: boolean
	lastCachedAt: Date | null
	posts: Post[]
}

interface PrismaPostWithMetadata extends PrismaPost {
	tags: PrismaTag[]
	blog: PrismaBlog
	author: PrismaUser
	likes: any
}

interface PrismaFeedWithPosts extends PrismaFeed {
	posts: PrismaPostWithMetadata[]
}

const orderTranslator: Translator<Order, string> = {
	toModel: (prisma) => prisma,
	toPrisma: (model) => {
		switch (model) {
			case "ASC":
				return Order.ASC
			case "DESC":
				return Order.DESC
			case "NONE":
				return Order.NONE
			default:
				console.error(`Attempted to translate invalid order ${model}`)
				return Order.NONE
		}
	},
}

export const feedTranslator: Translator<PrismaFeedWithPosts, Feed> = {
	toModel: (prisma) => {
		return {
			id: prisma.id,
			slug: prisma.slug,
			name: prisma.name,
			publishOrder: prisma.publishOrder,
			filterTags: prisma.filterTags,
			// TODO add tags
			lastCachedAt: prisma.lastCachedAt,
			posts: prisma.posts.map((post: PrismaPostWithMetadata) =>
				postTranslator.toModel(post)
			),
		}
	},
	toPrisma: (model) => ({
		id: model.id,
		slug: model.slug,
		name: model.name,
		publishOrder: orderTranslator.toPrisma(model.publishOrder),
		filterTags: model.filterTags,
		lastCachedAt: model.lastCachedAt,
		posts: model.posts.map((post) => postTranslator.toPrisma(post)),
	}),
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
		})

		return await Promise.all(
			feeds.map(async (feed) =>
				feedTranslator.toModel(await fetchPosts(feed))
			)
		)
	},

	bySlug: async (slug: string): Promise<Feed> => {
		const feed = (await db.feed.findUnique({
			where: {
				slug,
			},
		})) as unknown as PrismaFeedWithPosts

		return feedTranslator.toModel(await fetchPosts(feed))
	},
}

const fetchPosts = async (feed: PrismaFeed): Promise<PrismaFeedWithPosts> => {
	let posts: PrismaPostWithMetadata[] = []

	// Special cases for new and popular feeds
	if (feed.slug === "new") {
		posts = await db.post.findMany({
			take: 10,
			orderBy: {
				created: "desc",
			},
			include: {
				author: true,
				tags: true,
				blog: true,
				likes: true,
			},
		})
	} else if (feed.slug === "popular") {
		posts = await db.post.findMany({
			take: 10,
			orderBy: {
				likes: {
					_count: "desc",
				},
			},
			include: {
				author: true,
				tags: true,
				blog: true,
				likes: true,
			},
		})
	}

	return { ...feed, posts }
}
