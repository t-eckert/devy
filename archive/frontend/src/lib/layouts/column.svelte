<script lang="ts">
	import { Snippet } from "svelte"

	let { children }: { children: Snippet } = $props()
</script>

<div class="mx-auto flex flex-col gap-2 sm:gap-3">
	{@render children()}
</div>
