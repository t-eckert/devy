<script lang="ts">
	import type { Blog } from "$lib/types"

	import Button from "$lib/components/button.svelte"
	import Plus from "$lib/icons/plus.svelte"

	interface Props {
		blog: Blog
	}

	const { blog }: Props = $props()
</script>

<section class="my-3 flex flex-col gap-2">
	<h1 class="text-3xl sm:text-5xl font-bold break-normal text-zinc-900 sm:leading-tight">
		{blog.name}
	</h1>
	<div class="flex flex-row items-baseline justify-between mt-1">
		<Button role="secondary">
			<span><Plus /></span>
			<span class="text-sm font-medium">Follow</span>
		</Button>
	</div>
</section>
