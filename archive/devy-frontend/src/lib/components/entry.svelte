<script lang="ts">
	import type { Entry } from "$lib/types"
	import Likes from "$lib/components/likes.svelte"
	import RelativeDate from "$lib/components/relative-date.svelte"
	import Bookmark from "$lib/components/bookmark.svelte"

	type Omitable = "author" | "blog"

	interface Props {
		entry: Entry
		omit?: Omitable[]
	}

	const { entry }: Props = $props()
</script>

<div class="w-full max-w-xl">
	<section class="rounded border border-stone-100 bg-white shadow-sm transition-all hover:shadow">
		{#if entry.coverImage}
			<div class="rounded-t">
				<a href={`/${entry.blogSlug}/${entry.slug}`}>
					<img
						src={entry.coverImage || "/og-img.png"}
						alt={`Cover image for ${entry.title}`}
						class="rounded-t"
					/>
				</a>
			</div>
		{/if}

		<div class="grid grid-cols-8 px-2 py-1.5">
			<div class="col-span-7 col-start-2">
				<a href={`/${entry.blogSlug}`} class="text-sm text-stone-600 hover:text-stone-950"
					>{entry.blogName}</a
				>
			</div>

			<div class="col-span-1 flex flex-row items-center justify-center">
				<Likes entryId={entry.id} likeCount={entry.likeCount} />
			</div>
			<div class="col-span-7 flex flex-row items-center gap-1.5">
				{#if entry.isDraft}
					<span class="font-medium text-yellow-600"> Draft: </span>
				{/if}
				<a
					href={`/${entry.blogSlug}/${entry.slug}`}
					class="font-medium text-stone-800 hover:text-stone-950"
				>
					<h1>{entry.title}</h1>
				</a>
			</div>

			<div class="col-span-7 col-start-2 flex flex-row gap-1.5">
				<a
					href={`/profiles/${entry.authorSlug}`}
					class="text-sm text-stone-600 transition-all hover:text-stone-950 dark:text-stone-400"
					>{entry.authorName}</a
				>
				<div class="text-sm text-stone-700">
					<RelativeDate date={entry.createdAt} />
				</div>
				<div>
					<Bookmark postId={entry.id} />
				</div>
			</div>
		</div>
	</section>
</div>
