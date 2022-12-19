import { marked } from "marked"

export default function parse(markdown: string): string {
	return marked.parse(markdown)
}
