<script lang="ts">
	import type { Blog } from "$lib/types"

	import Button from "$lib/components/button.svelte"
	import Plus from "$lib/icons/plus.svelte"

	interface Props {
		blog: Blog
		isUserFollowing: boolean
	}

	const { blog, isUserFollowing }: Props = $props()
</script>

<section class="my-3 flex flex-col gap-2">
	<h1 class="text-3xl sm:text-5xl font-semibold break-normal text-zinc-900 sm:leading-tight">
		{blog.name}
	</h1>
	<form class="flex flex-row items-baseline justify-between mt-1" method="POST" action="?/follow">
		<input name="blogId" value={blog.id} class="hidden" />
		{#if isUserFollowing}
			<Button role="tertiary" name="action" value="unfollow">
				<span class="text-sm font-medium">Unfollow</span>
			</Button>
		{:else}
			<Button role="secondary" name="action" value="follow">
				<span><Plus /></span>
				<span class="text-sm font-medium">Follow</span>
			</Button>
		{/if}
	</form>
</section>
