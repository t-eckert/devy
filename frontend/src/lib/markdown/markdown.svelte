<script lang="ts">
	import parser from "./parser"
	import Node from "./node.svelte"

	export let source: string

	$: node = parser(source)
</script>

<Node {node} />
