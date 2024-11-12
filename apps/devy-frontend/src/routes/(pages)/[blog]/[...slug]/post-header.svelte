<script lang="ts">
	import { cva } from "cva"
	import type { Post } from "$lib/types"
	import Likes from "$lib/components/likes.svelte"
	import RelativeDate from "$lib/components/relative-date.svelte"
	import Bookmark from "$lib/components/bookmark.svelte"

	interface Props {
		post: Post
	}

	const { post }: Props = $props()

	let titleIsLong = $derived(post.title.length > 30)

	const title = cva("font-semibold text-stone-800 transition-all", {
		variants: {
			longTitle: {
				true: "text-4xl my-3",
				false: "text-5xl my-4"
			}
		}
	})
</script>

<div class="mx-auto my-8 w-full max-w-3xl transition-all">
	{#if post.coverImage}
		<div class="mb-6 rounded shadow-sm">
			<img
				src={post.coverImage || "/og-img.png"}
				alt={`Cover image for ${post.title}`}
				class="rounded"
			/>
		</div>
	{/if}

	<a href={`/${post.blogSlug}`} class="text-sm text-stone-600 hover:text-stone-950"
		>{post.blogName}</a
	>

	<h1 class={title({ longTitle: titleIsLong })}>
		{#if post.isDraft}
			<span class="font-medium text-yellow-500"> Draft: </span>
		{/if}
		{post.title}
	</h1>

	<div class="flex flex-row gap-1.5">
		<a
			href={`/profiles/${post.authorSlug}`}
			class="text-sm text-stone-600 transition-all hover:text-stone-950 dark:text-stone-400"
			>{post.authorName}</a
		>
		<div class="text-sm text-stone-700">
			<RelativeDate date={post.createdAt} />
		</div>
	</div>

	<div class="mt-4 flex flex-row">
		<Likes entryId={post.id} likeCount={post.likeCount} />
		<Bookmark postId={post.id} />
	</div>
</div>
