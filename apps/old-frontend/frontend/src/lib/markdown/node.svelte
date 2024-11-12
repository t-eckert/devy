<script lang="ts">
	import type { RenderableTreeNodes } from "@markdoc/markdoc"
	import renderer, { doesRender } from "./renderers"
	import Codeblock from "$lib/components/codeblock.svelte"
	import Json from "$lib/utils/json.svelte"

	let { node }: { node: RenderableTreeNodes } = $props()

	let isNull = $derived(node === null)
	let isPrimitive = $derived(typeof node !== "object")
	let isIterable = $derived(!isNull && !isPrimitive && typeof node[Symbol.iterator] === "function")
	let isRenderable = $derived(doesRender(node))
	let isCodeblock = $derived(
		node.name === "pre" && Object.keys(node.attributes).includes("data-language")
	)
	let RenderedComponent = $derived(renderer(node))
</script>

{#if isPrimitive}
	{node}
{:else if isCodeblock}
	<Codeblock lang={node.attributes["data-language"]} code={node.children[0]} />
{:else if isRenderable && node !== null}
	<RenderedComponent {...node.attributes}>
		<svelte:self node={node.children} />
	</RenderedComponent>
{:else if isIterable}
	{#each node as n}
		<svelte:self node={n} />
	{/each}
{:else}
	<svelte:element this={node.name} {...node.attributes}>
		<svelte:self node={node.children} />
	</svelte:element>
{/if}
