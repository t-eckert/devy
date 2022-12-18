import { marked } from "marked"
import DOMPurify from "dompurify"

export default function parse(markdown: string): string {
	return DOMPurify.sanitize(marked.parse(markdown))
}
