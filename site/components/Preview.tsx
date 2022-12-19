import { PostMetadata } from "interfaces"
import Link from "./Link"

type Props = {
	postMetadata: PostMetadata
}

const Preview: React.FC<Props> = ({ postMetadata }) => {
	return (
		<div>
			<Link href="#">
				<h2 className="font-medium">{postMetadata.title}</h2>
			</Link>
		</div>
	)
}

export default Preview
