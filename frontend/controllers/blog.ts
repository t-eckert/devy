import Blog from "@/models/Blog"
import config from "@/config"

const get = {
	bySlug: async (slug: string): Promise<Option<Blog>> => {
		try {
			const res = await fetch(`${config.API}/blogs/${slug}`, {
				next: { revalidate: 0 },
			})

			if (!res.ok) return null

			return await res.json()
		} catch (e) {
			console.error(e)
			return null
		}
	},
}

const upsert = async (blog: Blog): Promise<Option<Blog>> => {
	return null
}

const blog_controller = {
	get,
	upsert,
}

export default blog_controller
