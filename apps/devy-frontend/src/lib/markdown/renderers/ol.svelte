<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<ol {...props} class="ml-5 mt-2 flex list-outside list-decimal flex-col gap-2">
	{@render children()}
</ol>
