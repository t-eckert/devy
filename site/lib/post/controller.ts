import { Post, PostMetadata } from "./Post"
import { defaults } from "lib/feed"
import client from "client"
import Profile from "lib/profile"

export const getPostByPath = async (
	authorUsername: string,
	slug: string
): Promise<Post> => {
	const { data, error } = await client
		.from("post")
		.select(
			"created_at, updated_at, title, slug, markdown, author (username, name)"
		)
		.eq("slug", slug)
		.eq("author.username", authorUsername)
		.limit(1)
		.single()

	if (error) {
		throw error
	}

	const post: Post = {
		createdAt: data.created_at,
		updatedAt: data.updated_at,
		title: data.title,
		slug: data.slug,
		markdown: data.markdown,
		author: data.author as Profile,
		tags: [],
		likes: 0,
	}

	return post
}

export const getPostsMetadataByFeed = async (
	feedId: string
): Promise<PostMetadata[]> => {
	if (!defaults.map((d) => d.id).includes(feedId)) {
		throw "Not found"
	}

	const popular = defaults.at(0)
	if (feedId === popular?.id) {
		const { data, error } = await client.from("post").select("*").limit(10)
		if (error) {
			throw error
		}

		return data as unknown as PostMetadata[]
	}

	const newFeed = defaults.at(1)
	if (feedId === newFeed?.id) {
		const { data, error } = await client.from("post").select("*").limit(10)
		if (error) {
			throw error
		}

		return data as unknown as PostMetadata[]
	}

	return []
}

export const getPostsMetadataByPopularity = async (): Promise<
	PostMetadata[]
> => {
	const { data, error } = await client.from("post").select("*").limit(10)

	if (error) {
		throw error
	}

	return data as unknown as PostMetadata[]
}

export const getNewPostsMetadata = async (): Promise<PostMetadata[]> => {
	const { data, error } = await client.from("post").select("*").limit(10)

	if (error) {
		throw error
	}

	return data as unknown as PostMetadata[]
}
