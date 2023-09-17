import { Link } from "@/components/elements"

import { marked } from "marked"

interface Props {
	changelog: string
}

export default function Changelog({ changelog }: Props) {
	const tokens = marked.lexer(changelog)

	return <section className="flex flex-col">
		<h1 className="mb-1">Changelog</h1>
		{tokens.map(formatToken)}
	</section>
}

// This heavily relies on the structure of the changelog.
const formatToken = (token: any, _: number): Option<React.ReactNode> => {
	switch (token.type) {
		case "heading":
			switch (token.depth) {
				case 2:
					return (
						<div className="hover:pl-2 border-l-zinc-700 transition-all hover:border-l">
							<Link href="/changelog">
								<h2 className="mb-1 flex flex-col">
									<span className="text-zinc-500 text-xs font-mono">
										{token.text.split(" ")[0].replaceAll("`", "")}
									</span>
									<span className="font-medium text-zinc-300 text-sm">
										{token.text
											.split(" ")
											.slice(1)
											.join(" ")}
									</span>
								</h2>
							</Link>
						</div>
					)
				default:
					return null
			}
		default:
			return null
	}
}
