import { Link } from "@/components/elements"

import { marked } from "marked"

interface Props {
	changelog: string
}

export default function Changelog({ changelog }: Props) {
	const tokens = marked.lexer(changelog)

	return <div>{tokens.map(formatToken)}</div>
}

// This heavily relies on the structure of the changelog.
const formatToken = (token: any, _: number): Option<React.ReactNode> => {
	switch (token.type) {
		case "heading":
			switch (token.depth) {
				case 2:
					return (
						<div className="pl-2 border-l-zinc-500 border-l">
							<h2 className="mb-1 flex flex-col text-sm">
								<span className="text-zinc-500">
									{token.text.split(" ")[0]}
								</span>
								<span className="font-medium text-zinc-300 transition-all hover:underline">
									<Link href="/changelog">
										{token.text
											.split(" ")
											.slice(1)
											.join(" ")}
									</Link>
								</span>
							</h2>
						</div>
					)
				default:
					return null
			}
		default:
			return null
	}
}
