import PostPreview from "@/components/post-preview"
import Button from "@/components/button"

import { Post } from "@/models"

interface Props {
	posts: Post[]
	loadMore?: () => void
}

export default function PostCollection({ posts, loadMore }: Props) {
	return (
		<section className="w-full max-w-2xl flex flex-col gap-4">
			{posts.map((post, index) => (
				<PostPreview key={index} post={post} />
			))}
			<div className="border-t border-t-neutral-medium py-4 w-full flex items-center justify-center">
				{loadMore && <Button onClick={loadMore}>Load more</Button>}
			</div>
		</section>
	)
}
