import Blog from "@/models/Blog"
import config from "@/config"

const get = {
	bySlug: async (slug: string): Promise<Option<Blog>> => {
		const res = await fetch(`${config.API}/blogs/${slug}`, {
			next: { revalidate: 0 },
		})

		if (!res.ok) return null

		return await res.json()
	},
}

const upsert = async (blog: Blog): Promise<Option<Blog>> => {
	return null
}

const blog: Controller<Blog> = {
	get,
	upsert,
}

export default blog
