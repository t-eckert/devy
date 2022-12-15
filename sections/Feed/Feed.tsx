import Preview from "components/Preview"
import { PostMetadata } from "interfaces"

type Props = {
	postsMetadata?: PostMetadata[]
}

const Feed: React.FC<Props> = ({ postsMetadata }) => {
	return (
		<section className="flex flex-col">
			<div className="flex flex-row justify-between">
				<ul className="flex flex-row gap-2">
					<li className="underline underline-offset-2">Popular</li>
					<li>New</li>
				</ul>
				<div>Search</div>
			</div>
			{postsMetadata?.map((postMetadata, index) => (
				<Preview key={index} postMetadata={postMetadata} />
			))}
		</section>
	)
}

export default Feed
