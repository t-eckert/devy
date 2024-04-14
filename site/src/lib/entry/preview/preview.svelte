<script lang="ts">
	import type { Entry } from "$lib/types"
	import RelativeDate from "$lib/utils/relative-date.svelte"
	import { Heart, Bookmark } from "radix-icons-svelte"

	export let entry: Entry
	const { postSlug, title, blogSlug, blogName, authorName, authorSlug, likes, createdAt } = entry
</script>

<div class="grid grid-cols-10 max-w-xl">
	<div class="col-start-1 row-start-2">
		<button class="text-sm flex flex-row items-center gap-1 group"
			><Heart
				class="w-4 aspect-square text-zinc-500 group-hover:-translate-y-1 transition-all group-hover:rotate-12 group-hover:scale-110"
			/>{likes}</button
		>
	</div>
	<div class="col-start-2 row-start-1 col-span-9 -mb-0.5">
		<a href={`${blogSlug}`} class="text-xs text-zinc-500">{blogName}</a>
	</div>
	<div class="col-start-2 row-start-2 col-span-9">
		<a href={`${blogSlug}/${postSlug}`} class="font-medium">{title}</a>
	</div>
	<div class="col-start-2 row-start-3 flex flex-row gap-2 col-span-9">
		<a href={`/profiles/${authorSlug}`} class="text-xs text-zinc-500">{authorName}</a>
		<div class="text-xs"><RelativeDate date={createdAt} /></div>
		<button class="rounded group"
			><Bookmark
				class="w-4 aspect-square text-zinc-500 group-hover:-translate-y-1 group-hover:text-zinc-950 transition-all"
			/></button
		>
	</div>
</div>
