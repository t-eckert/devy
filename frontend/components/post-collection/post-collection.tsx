import PostPreview from "@/components/post-preview"
import Button from "@/components/button"

import { Post } from "@/models"

interface Props {
	posts: Post[]
	next?: () => void
	prev?: () => void
}

export default function PostCollection({ posts, next, prev }: Props) {
	return (
		<section className="w-full max-w-2xl flex flex-col gap-4">
			{posts.map((post, index) => (
				<PostPreview key={index} post={post} />
			))}
			<div className="border-t border-t-neutral-medium py-4 w-full flex flex-row-reverse justify-between">
				{next && <Button onClick={next}>Next</Button>}
				{prev && <Button onClick={prev}>Previous</Button>}
			</div>
		</section>
	)
}
