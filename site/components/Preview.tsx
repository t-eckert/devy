import { PostMetadata } from "interfaces"

type Props = {
	postMetadata: PostMetadata
}

const Preview: React.FC<Props> = ({ postMetadata }) => {
	return (
		<div>
			<h2>{postMetadata.title}</h2>
		</div>
	)
}

export default Preview
