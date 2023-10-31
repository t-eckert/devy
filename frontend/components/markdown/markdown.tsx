import { MDXRemote } from "next-mdx-remote/rsc"

import components from "./components"

interface Props {
	content: string
}

export default function Markdown({ content }: Props) {
	return <MDXRemote source={content} components={components} />
}
