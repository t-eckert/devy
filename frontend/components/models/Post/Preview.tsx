import Link from "next/link"

import Post from "@/models/Post"
import Json from "@/components/debugging/Json"

interface Props extends Post {}

export default function Preview(post: Props) {
	return (
		<div className="flex flex-row items-start gap-3">
			<div className="flex flex-col">
				<Json data={post} />
			</div>
		</div>
	)
}
