<script lang="ts">
	import parser from "./parser"
	import Node from "./node.svelte"

	let { source }: { source: string } = $props()
	let node = $derived(parser(source))
</script>

<Node {node} />
