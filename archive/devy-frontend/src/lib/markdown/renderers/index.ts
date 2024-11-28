import { type RenderableTreeNode, type RenderableTreeNodes } from "@markdoc/markdoc"
import { type Component } from "svelte"

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
import Td from "./td.svelte"
import Thead from "./thead.svelte"
import Th from "./th.svelte"
import Tr from "./tr.svelte"

export const renderers: Map<string, Component> = new Map([
	["h1", H1],
	["h2", H2],
	["h3", H3],
	["h4", H4],
	["h5", H5],
	["p", P],
	["img", Image],
	["article", Article],
	["ol", Ol],
	["ul", Ul],
	["li", Li],
	["pre", Pre],
	["a", Link],
	["blockquote", Blockquote],
	["strong", Strong],
	["s", Strikethrough],
	["em", Em],
	["code", Code],
	["td", Td],
	["thead", Thead],
	["table", Table],
	["th", Th],
	["tr", Tr]
])

export function doesRender(node: RenderableTreeNodes): boolean {
	if (node === null) return false
	if (typeof node !== "object") return false

	if (Object.keys(node).includes("name")) return renderers.has(node.name)

	return false
}

export default function render(node: RenderableTreeNode): Component | undefined {
	if (node === null) {
		throw Error("No node passed in to renderer")
	}

	const component = renderers.get(node.name)

	return component
}
