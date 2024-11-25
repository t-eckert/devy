import Markdoc, { type RenderableTreeNode } from "@markdoc/markdoc"

export default function parse(source: string): RenderableTreeNode {
	const ast = Markdoc.parse(source)
	return Markdoc.transform(ast)
}
