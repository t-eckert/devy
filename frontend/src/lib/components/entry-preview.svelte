<script lang="ts">
	import type { Entry } from "$lib/types"
	import readingTime from "$lib/readingTime"

	import RelativeDate from "$lib/utils/relative-date.svelte"
	import Likes from "$lib/components/likes.svelte"
	import Bookmark from "$lib/components/bookmark.svelte"

	const Date = "date"
	const Author = "author"
	const Blog = "blog"

	interface Props {
		entry: Entry
		omit?: string[]
	}

	const { entry, omit }: Props = $props()

	const defaults = [Date, Author, Blog]
	const components = new Set(defaults.filter((component) => !omit?.includes(component)))
</script>

<div class="max-w-xl">
	<section
		class="h-full w-full rounded border border-stone-200/70 backdrop-blur-3xl shadow bg-white/60 dark:bg-zinc-800/60 dark:border-zinc-800 dark:shadow-none px-2 py-1.5"
	>
		<div class="flex flex-col">
			{#if components.has(Blog)}
				<div class="ml-[4.75rem]">
					<a
						href={`/${entry.blogSlug}`}
						class="text-xs text-stone-500 hover:text-stone-600 dark:text-stone-400"
						>{entry.blogName}</a
					>
				</div>
			{/if}

			<div class="flex flex-row gap-1 items-start">
				<div class="ml-2 w-16 mt-[0.12rem]">
					<Likes postId={entry.id} likes={entry.likeCount} />
				</div>
				<div class="flex flex-col gap-1">
					<a
						href={`/${entry.blogSlug}/${entry.postSlug}`}
						class="font-medium text-stone-700 hover:text-stone-950 dark:text-stone-200 transition-all"
						>{entry.title}</a
					>

					<div class="flex flex-row gap-2">
						{#if components.has(Author)}
							<a
								href={`/profiles/${entry.authorSlug}`}
								class="text-xs text-stone-700 hover:text-stone-950 dark:text-stone-400 transition-all"
								>{entry.authorName}</a
							>
						{/if}
						<div class="text-xs text-stone-800 dark:text-stone-300">
							<RelativeDate date={entry.createdAt} />
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
