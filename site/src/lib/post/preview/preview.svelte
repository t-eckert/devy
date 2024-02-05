<script lang="ts">
	import type { Post } from "$lib/types"
	import Json from "$lib/utils/json.svelte"
	import { Slash } from "radix-icons-svelte"
	import H2 from "$lib/element/h2.svelte"
	import Like from "$lib/post/like/like.svelte"
	import RelativeDate from "$lib/utils/relative-date.svelte"

	export let post: Post
	const { title, slug, authorName, authorUsername, blogName, blogSlug, createdAt } = post
</script>

<div class="grid grid-rows-3 grid-cols-[max-content_1fr] gap-x-4 auto-rows-min">
	<div class="col-start-1 row-start-2 w-10 flex flex-row justify-end">
		<Like {post} />
	</div>
	<div class="col-start-2 col-span-3 row-start-1 flex flex-row items-end gap-0.5 text-xs">
		<a href={`/profiles/${authorUsername}`} class="text-zinc-600">{authorName}</a>
		<Slash class="text-zinc-500" />
		<a href={"#"} class="text-zinc-600">{blogName}</a>
	</div>
	<div class="col-start-2 col-span-3 row-start-2 font-medium">
		<H2><a href={`/${blogSlug}/${slug}`}>{title}</a></H2>
	</div>
	<div class="col-start-2 col-span-3 row-start-3 text-xs"><RelativeDate date={createdAt} /></div>
</div>
