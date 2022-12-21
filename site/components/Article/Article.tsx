import DOMPurify from "isomorphic-dompurify"

export interface Props {
	html: string
}

const Article: React.FC<Props> = ({ html }) => {
	return (
		<article dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(html) }} />
	)
}

export default Article
