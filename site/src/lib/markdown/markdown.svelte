<script lang="ts">
	import parser from "./parser"
	import Json from "$lib/utils/json.svelte"
	import Node from "./node.svelte"

	export let source: string

	$: node = parser(source)
</script>

<Node {node} />
