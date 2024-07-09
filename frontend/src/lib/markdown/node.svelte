<script lang="ts">
	import type { RenderableTreeNode, RenderableTreeNodes } from '@markdoc/markdoc';
	import render from './renderers';

	function isIterable(obj: RenderableTreeNodes): boolean {
		return obj !== null && typeof obj === 'object' && typeof obj[Symbol.iterator] === 'function';
	}

	function isRoot(obj: RenderableTreeNodes) {
		return !isIterable(obj) && Object.keys(obj).includes('children');
	}

	function hasChildren(obj: RenderableTreeNodes) {
		return Object.keys(obj).includes('children');
	}

	export let node: RenderableTreeNodes;
</script>

{#if node !== null}
	{#if typeof node === 'string'}
		{node}
	{:else if isIterable(node)}
		{#each node as n}
			<svelte:self node={n} />
		{/each}
	{:else if !isIterable(node)}
		{#if render(node)}
			<svelte:component this={render(node)} {node}>
				<svelte:self node={node.children} />
			</svelte:component>
		{:else if typeof node === 'string'}
			{node}
		{:else if hasChildren(node)}
			<svelte:element this={node.name}>
				<svelte:self node={node.children} />
			</svelte:element>
		{/if}
	{/if}
{/if}
