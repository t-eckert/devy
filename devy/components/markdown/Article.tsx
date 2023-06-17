import { MDXRemote } from "next-mdx-remote/rsc"

interface Props {
	markdown: string
}

export default async function Article({ markdown }: Props) {
	return (
		<article>
			<MDXRemote source={markdown} />
		</article>
	)
}
