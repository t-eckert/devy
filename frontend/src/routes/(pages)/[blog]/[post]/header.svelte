<script lang="ts">
	import type { Post } from "$lib/types"
	import RelativeDate from "$lib/utils/relative-date.svelte"
	import Likes from "$lib/components/likes.svelte"

	let { post }: { post: Post } = $props()
</script>

<div class="my-3 mx-auto w-full max-w-3xl flex flex-col gap-4">
	<div class="w-full">
		<a href={`/${post.blogSlug}`} class="text-sm text-stone-500">{post.blogName}</a>
	</div>
	<h1 class="text-3xl sm:text-5xl font-bold break-normal text-zinc-900 sm:leading-tight">
		{post.title}
	</h1>
	<div class="flex flex-row gap-2">
		<a href={`/profiles/${post.authorSlug}`} class="text-sm text-stone-500">{post.authorName}</a>
		<div class="text-sm dark:text-stone-300">
			<RelativeDate date={post.createdAt} />
		</div>
	</div>
	<div class="flex flex-row">
		<Likes postId={post.id} likes={post.like_count} />
	</div>

	{#if post.isDraft}
		<div class="flex flex-col gap-1 items-start">
			<span class="text-xs bg-yellow-500 px-1 py-0.5 rounded font-medium text-white">DRAFT</span>
			<div class="text-sm flex flex-col gap-1">
				<p>
					This post is a draft. Draft posts will not be listed in feeds, blogs, or on your profile.
					They are publicly visible to anyone with the URL.
				</p>
				<p>
					You can control whether or not your post is in draft my using the <code
						class="bg-stone-200 text-[0.9em]">draft: true</code
					>
					frontmatter in your markdown.
				</p>
			</div>
		</div>
	{/if}
</div>
