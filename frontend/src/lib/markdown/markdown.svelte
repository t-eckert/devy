<script lang="ts">
	import parser from "./parser"
	import Window from "$lib/utils/window.svelte"
	import Json from "$lib/utils/json.svelte"
	import Node from "./node.svelte"

	export let source: string

	$: node = parser(source)
</script>

<Node {node} />
