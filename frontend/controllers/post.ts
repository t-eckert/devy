import Post from "@/models/Post"
import config from "@/config"

const get = {
	byBlogSlugAndPostSlug: async (
		blog: string,
		post: string
	): Promise<Option<Post>> => {
		try {
			const res = await fetch(
				`${config.API}/blogs/${blog}/posts/${post}`,
				{
					next: { revalidate: 10 },
				}
			)

			if (!res.ok) return null

			return await res.json()
		} catch (e) {
			console.log(e)
			return null
		}
	},

	byBlogSlug: async (blog: string): Promise<Option<Post>> => {
		try {
			const res = await fetch(`${config.API}/blogs/${blog}/posts`, {
				next: { revalidate: 10 },
			})

			if (!res.ok) return null

			return await res.json()
		} catch (e) {
			console.log(e)
			return null
		}
	},

	byFeed: async (id: string): Promise<Option<Post[]>> => {
		try {
			const res = await fetch(`${config.API}/feeds/${id}/posts`, {
				next: { revalidate: 10 },
			})

			if (!res.ok) return null

			return await res.json()
		} catch (e) {
			console.log(e)
			return null
		}
	},
}

const upsert = async (_: Post): Promise<Option<Post>> => {
	return null
}

const post: Controller<Post> = {
	get,
	upsert,
}

export default post
