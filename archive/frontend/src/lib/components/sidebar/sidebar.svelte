<script lang="ts">
	import type { Snippet } from "svelte"

	interface Props {
		children: Snippet
	}

	const { children }: Props = $props()
</script>

<section
	class="flex flex-row sm:flex-col gap-1 sm:gap-1 sticky top-4 pb-2 overflow-scroll sm:overflow-auto"
>
	{@render children()}
</section>
