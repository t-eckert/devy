import { RenderableTreeNode } from "@markdoc/markdoc"
import { ComponentType } from "svelte"

import Article from "./article.svelte"
import H1 from "./h1.svelte"
import H2 from "./h2.svelte"
import H3 from "./h3.svelte"
import H4 from "./h4.svelte"
import H5 from "./h5.svelte"
import P from "./p.svelte"
import Image from "./image.svelte"
import Ol from "./ol.svelte"
import Ul from "./ul.svelte"
import Li from "./li.svelte"
import Pre from "./pre.svelte"
import Link from "./link.svelte"
import Blockquote from "./blockquote.svelte"
import Strong from "./strong.svelte"
import Strikethrough from "./strikethrough.svelte"
import Em from "./em.svelte"
import Code from "./code.svelte"
import Table from "./table.svelte"

const renderers: Record<string, ComponentType> = {
	h1: H1,
	h2: H2,
	h3: H3,
	h4: H4,
	h5: H5,
	p: P,
	img: Image,
	article: Article,
	ol: Ol,
	ul: Ul,
	li: Li,
	pre: Pre,
	a: Link,
	blockquote: Blockquote,
	strong: Strong,
	s: Strikethrough,
	em: Em,
	code: Code,
	table: Table
}

export default function render(node: RenderableTreeNode): ComponentType | null {
	if (node === null) return null

	return renderers[node?.name]
}
