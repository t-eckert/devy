<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<th {...props} class="font-medium text-slate-900">{@render children()}</th>
