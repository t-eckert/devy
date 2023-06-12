import db from "@/db"

import User from "./User"

export default interface Post {
	slug: string
	blog: string
	author?: User
	title?: string
	published?: string
	updated?: string
	tags?: string[]
	likes?: number
	markdown: string
}

export const getPostsByUser = async (username: string): Promise<Post[]> => {
	return []
}

export const getPostByUserAndSlug = async (
	username: string,
	slug: string
): Promise<Post | null> => {
	const post = await db.post.findFirst({
		where: {
			author: {
				username,
			},
			slug,
		},
	})

	if (!post) {
		return null
	}

	return {
		slug: post.slug,
		blog: post.blogSlug,
		markdown: post.markdown ?? "",
	}
}

export const getPostByBlogAndSlug = async (blog: string, slug: string) => {
	const post = await db.post.findFirst({
		where: {
			blog: {
				slug: blog,
			},
			slug,
		},
	})

	if (!post) {
		return null
	}

	return {
		slug: post.slug,
		blog: post.blogSlug,
		markdown: post.markdown ?? "",
	}
}

export const upsertPost = async (post: Post): Promise<Post> => {
	return post
}
