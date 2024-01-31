import PostPreview from "@/components/post/preview"

import { Post } from "@/models"

interface Props {
	posts: Post[]
}

export default function Collection({ posts }: Props) {
	return (
		<section className="w-full max-w-2xl flex flex-col gap-1">
			{posts.map((post, index) => (
				<PostPreview key={index} post={post} />
			))}
		</section>
	)
}
