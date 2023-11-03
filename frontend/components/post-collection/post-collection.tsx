import { Post } from "@/models"
import PostPreview from "@/components/post-preview"

interface Props {
	posts: Post[]
	next?: () => void
	prev?: () => void
}

export default function PostCollection({ posts, next, prev }: Props) {
	return (
		<section>
			{posts.map((post, index) => (
				<PostPreview key={index} post={post} />
			))}
		</section>
	)
}

