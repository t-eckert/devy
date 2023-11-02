import { Post } from "@/models"
import PostPreview from "@/components/post-preview"

interface Props {
	posts: Post[]
	next: Adjacent
	prev: Adjacent
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

type Adjacent = {
	exists: boolean
	onNavigate: () => void
}
