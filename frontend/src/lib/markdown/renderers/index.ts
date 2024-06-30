import { RenderableTreeNode } from "@markdoc/markdoc"
import { ComponentType } from "svelte"

import H1 from "$lib/elements/h1.svelte"
import H2 from "$lib/elements/h2.svelte"
import H3 from "$lib/elements/h3.svelte"
import H4 from "$lib/elements/h4.svelte"
import H5 from "$lib/elements/h5.svelte"
import P from "$lib/elements/p.svelte"
import Image from "$lib/elements/image.svelte"
import Ol from "$lib/elements/ol.svelte"
import Ul from "$lib/elements/ul.svelte"
import Li from "$lib/elements/li.svelte"
import Pre from "$lib/elements/pre.svelte"
import Link from "$lib/elements/link.svelte"
import Blockquote from "$lib/elements/blockquote.svelte"
import Strong from "$lib/elements/strong.svelte"
import Strikethrough from "$lib/elements/strikethrough.svelte"
import Em from "$lib/elements/em.svelte"
import Code from "$lib/elements/code.svelte"
import Table from "$lib/elements/table.svelte"

import Article from "./article.svelte"

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
