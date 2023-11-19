import ReactMarkdown from "react-markdown"
import remarkGfm from "remark-gfm"

import components from "./components"

interface Props {
	content: string
}

export default function Markdown({ content }: Props) {
	return (
		<ReactMarkdown
			remarkPlugins={[remarkGfm]}
			components={components}
			className="w-full max-w-6xl text-neutral-1 dark:text-neutral+1"
		>
			{removeTitle(content)}
		</ReactMarkdown>
	)
}

const removeTitle = (content: string) => {
	const lines = content.split("\n")
	if (lines[0].startsWith("# ")) {
		lines.shift()
	}
	return lines.join("\n")
}
