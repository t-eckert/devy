import DOMPurify from "isomorphic-dompurify"

type Props = {
	html: string
}

const Article: React.FC<Props> = ({ html }) => {
	const a = DOMPurify.sanitize(html)

	return <article dangerouslySetInnerHTML={{ __html: html }} />
}

export default Article
