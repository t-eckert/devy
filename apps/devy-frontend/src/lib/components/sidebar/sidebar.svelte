<script lang="ts">
	import type { Snippet } from "svelte"

	interface Props {
		children: Snippet
	}

	const { children }: Props = $props()
</script>

<section
	class="sticky top-4 flex flex-row gap-1 overflow-scroll pb-2 sm:flex-col sm:gap-1 sm:overflow-auto"
>
	{@render children()}
</section>
