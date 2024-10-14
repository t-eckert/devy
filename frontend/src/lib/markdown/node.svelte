<script lang="ts">
	import type { RenderableTreeNodes } from "@markdoc/markdoc"
	import renderer, { doesRender } from "./renderers"

	let { node }: { node: RenderableTreeNodes } = $props()

	let isNull = $derived(node === null)
	let isPrimitive = $derived(typeof node !== "object")
	let isIterable = $derived(!isNull && !isPrimitive && typeof node[Symbol.iterator] === "function")
	let isRenderable = $derived(doesRender(node))
	let RenderedComponent = $derived(renderer(node))
</script>

{#if isPrimitive}
	{node}
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
