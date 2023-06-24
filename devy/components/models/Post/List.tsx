import Preview from "@/components/models/Post/Preview"
import Post from "@/models/Post"

interface Props {
	posts: Post[]
}

export default function List({ posts }: Props) {
	return (
		<section className="w-full max-w-2xl flex flex-col gap-4">
			{posts.map((post, i) => (
				<Preview key={i} {...post} />
			))}
		</section>
	)
}
