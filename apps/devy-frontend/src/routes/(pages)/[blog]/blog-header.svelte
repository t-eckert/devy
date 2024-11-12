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

<section class="my-12 flex flex-col gap-8">
	<div>
		<h1 class="break-normal text-3xl font-medium text-zinc-900 sm:text-7xl sm:leading-tight">
			{blog.name}
		</h1>
		<a href={`/profiles/${blog.authorUsername}`}>{blog.authorDisplayName}</a>
	</div>
	<div class="mt-1 flex flex-row items-baseline justify-between">
		<form method="POST" action="?/follow">
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
	</div>
</section>
