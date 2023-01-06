import Preview from "components/Preview"
import { PostMetadata } from "lib/post"
import Feed from "lib/feed"

export interface Props {
	postsMetadata?: PostMetadata[]
	feeds: Feed[]
	feedId: string
	setFeedId: (id: string) => void
	pageNumber: number
	setPageNumber: (n: number) => void
}

const FeedComponent: React.FC<Props> = ({
	postsMetadata,
	feeds,
	feedId,
	setFeedId,
}) => {
	return (
		<section className="flex flex-col gap-4">
			<div className="flex flex-row justify-between">
				<ul className="flex flex-row gap-2 font-medium">
					{feeds?.map((feed) => (
						<li
							key={feed.id}
							className={[
								"cursor-pointer transition-all",
								feedId === feed.id ? "underline" : "",
							].join(" ")}
							onClick={() => setFeedId(feed.id)}
						>
							{feed.name}
						</li>
					))}
				</ul>
			</div>
			<div className="flex flex-col gap-4">
				{postsMetadata?.map((postMetadata, index) => (
					<Preview key={index} postMetadata={postMetadata} />
				))}
			</div>
		</section>
	)
}

export default FeedComponent
