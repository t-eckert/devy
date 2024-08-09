<script lang="ts">
	import type { Entry } from "$lib/types"
	import Markdown from "$lib/markdown/markdown.svelte"

	let { entry }: { entry: Entry } = $props()
</script>

<div class="mx-auto w-full max-w-3xl">
	<Markdown source={entry.body} />
</div>
