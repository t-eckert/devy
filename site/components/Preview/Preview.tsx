import { PostMetadata } from "lib/post"
import Link from "components/Link"

export interface Props {
	postMetadata: PostMetadata
}

const Preview: React.FC<Props> = ({ postMetadata }) => {
	// const path = `/${postMetadata.author.username}/${postMetadata.slug}`

	return (
		<div className="flex flex-col justify-start align-start">
			<Link href={"/"} style="primary" className="-mb-0.5">
				<h2 className="font-medium">{postMetadata.title}</h2>
			</Link>

			<div className="flex flex-row gap-1.5 text-sm mb-0.5">
				{/* <Link */}
				{/* 	href={`/${postMetadata.author.username}/profile`} */}
				{/* 	style="secondary" */}
				{/* > */}
				{/* 	{postMetadata.author.name} */}
				{/* </Link> */}
				{/* <span>{new Date(postMetadata.updatedAt).toLocaleDateString()}</span> */}
			</div>

			<div className="flex flex-row flex-wrap gap-1">
				{/* {postMetadata.tags.map((tag, index) => ( */}
				{/* 	<Link */}
				{/* 		href={`/search?tag=${tag}`} */}
				{/* 		key={index} */}
				{/* 		className={[ */}
				{/* 			"text-xs text-neutral-900 border rounded-full px-2 py-0.5 hover:-translate-y-1 hover:shadow-lg", */}
				{/* 			index % 2 === 0 ? "hover:-rotate-2" : "hover:rotate-2", */}
				{/* 		].join(" ")} */}
				{/* 	> */}
				{/* 		{tag} */}
				{/* 	</Link> */}
				{/* ))} */}
			</div>
		</div>
	)
}

export default Preview
