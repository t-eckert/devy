<script lang="ts">
	import type { Entry } from "$lib/types"
	import RelativeDate from "$lib/utils/relative-date.svelte"
	import Likes from "$lib/components/likes.svelte"

	let { entry }: { entry: Entry } = $props()
</script>

<div class="my-3 mx-auto w-full max-w-3xl flex flex-col gap-4">
	<div class="w-full">
		<a href={`/${entry.blogSlug}`} class="text-sm text-stone-500">{entry.blogName}</a>
	</div>
	<h1 class="text-3xl sm:text-5xl font-bold break-normal text-zinc-900 sm:leading-tight">
		{entry.title}
	</h1>
	<div class="flex flex-row gap-2">
		<a href={`/profiles/${entry.authorSlug}`} class="text-sm text-stone-500">{entry.authorName}</a>
		<div class="text-sm dark:text-stone-300">
			<RelativeDate date={entry.createdAt} />
		</div>
	</div>
	<div class="flex flex-row">
		<Likes postId={entry.id} likes={entry.likes} />
	</div>
</div>
