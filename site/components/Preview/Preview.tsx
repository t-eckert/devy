import { PostMetadata } from "lib/post"
import Link from "components/Link"

export interface Props {
	postMetadata: PostMetadata
}

const Preview: React.FC<Props> = ({ postMetadata }) => {
	return (
		<div className="flex flex-col justify-start">
			<Link href={postMetadata.path}>
				<h2 className="font-medium">{postMetadata.title}</h2>
			</Link>
			<Link href={postMetadata.author.id} style="secondary">
				{postMetadata.author.name}
			</Link>
			<span className="text-sm">
				Updated: {new Date(postMetadata.updated).toLocaleDateString()}
			</span>
			<div className="flex flex-row flex-wrap gap-1">
				{postMetadata.tags.map((tag, index) => (
					<span
						key={index}
						className="text-xs text-white bg-neutral-800 rounded-md px-2 py-0.5"
					>
						{tag}
					</span>
				))}
			</div>
		</div>
	)
}

export default Preview
