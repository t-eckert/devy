<script lang="ts">
	import type { Snippet } from "svelte"

	interface Props {
		children: Snippet
		sidebar: Snippet
	}

	const { children, sidebar }: Props = $props()
</script>

<div class="mx-auto grid grid-cols-1 gap-2 sm:grid-cols-4 sm:gap-10">
	<div>
		{@render sidebar()}
	</div>
	<div class="sm:col-span-3 sm:col-start-2">
		{@render children()}
	</div>
</div>
