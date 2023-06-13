import db from "@/db"
import User, { userTranslator } from "@/models/User"
import type Translator from "@/interfaces/Translator"
import Blog, { blogTranslator } from "@/models/Blog"

export default interface Post {
	slug: string
	blog: Blog
	author: User
	title: string
	published: string
	updated: string
	tags: string[]
	likes: number
	markdown: string
}

export const postTranslator: Translator<any, Post> = {
	toModel: (prisma) => ({
		slug: prisma.slug,
		blog: blogTranslator.toModel(prisma.blog),
		author: userTranslator.toModel(prisma.author),
		title: prisma.title,
		published: prisma.created,
		updated: prisma.updated,
		tags: prisma.tags,
		likes: prisma.likes.length,
		markdown: prisma.markdown,
	}),
	toPrisma: (model) => ({}),
}

export const postCreator = {}

export const postGetter = {
	byBlogAndSlug: async (blog: string, slug: string): Promise<any | null> => {
		const post = await db.post.findFirst({
			where: {
				blog: {
					slug: blog,
				},
				slug,
			},
			include: {
				author: true,
				blog: true,
				tags: true,
				likes: true,
			},
		})

		if (!post) return null

		return postTranslator.toModel(post)
	},
}

export const postUpdater = {}

export const postDeletor = {}
