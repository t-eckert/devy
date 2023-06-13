import type Translator from "@/interfaces/Translator"
import type Post from "@/models/Post"
import type User from "@/models/User"

export default interface Blog {
	slug: string
	name: string
	author: User
	posts: Post[]
}

export const blogTranslator: Translator<any, Blog> = {
	toModel: (prisma) => ({
		slug: prisma.slug,
		name: prisma.name,
		author: prisma.author,
		posts: prisma.posts,
	}),
	toPrisma: (model) => ({}),
}
