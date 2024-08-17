<script lang="ts">
	import type { Entry } from "$lib/types"
	import readingTime from "$lib/readingTime"

	import Card from "$lib/components/card.svelte"
	import RelativeDate from "$lib/utils/relative-date.svelte"
	import Likes from "$lib/components/likes.svelte"
	import Bookmark from "$lib/components/bookmark.svelte"

	const Date = "date"
	const Author = "author"
	const Blog = "blog"
	const ReadingTime = "reading-time"

	const Component = {
		Date,
		Author,
		Blog,
		ReadingTime
	} as const

	type ComponentKey = (typeof Component)[keyof typeof Component]

	interface Props {
		entry: Entry
		omit?: ComponentKey[]
	}

	const { entry, omit }: Props = $props()

	const defaults = new Set<ComponentKey>([Date, Author, Blog, ReadingTime])
	const components = defaults.difference(new Set(omit))

	const {
		id: postId,
		title,
		blogName,
		blogSlug,
		postSlug,
		authorName,
		authorSlug,
		createdAt,
		likes
	} = entry
</script>

<div class="max-w-xl">
	<Card>
		<div class="flex flex-col">
			{#if components.has(Blog)}
				<div class="ml-20">
					<a
						href={`/${blogSlug}`}
						class="text-xs text-stone-500 hover:text-stone-600 dark:text-stone-400">{blogName}</a
					>
				</div>
			{/if}

			<div class="flex flex-row gap-2 items-start">
				<div class="ml-2 w-16 mt-[0.12rem]">
					<Likes {postId} {likes} />
				</div>
				<div class="flex flex-col gap-1">
					<a
						href={`/${blogSlug}/${postSlug}`}
						class="font-medium text-stone-700 hover:text-stone-950 dark:text-stone-200 transition-all"
						>{title}</a
					>

					<div class="flex flex-row gap-2">
						{#if components.has(Author)}
							<a
								href={`/profiles/${authorSlug}`}
								class="text-xs text-stone-700 hover:text-stone-950 dark:text-stone-400 transition-all"
								>{authorName}</a
							>
						{/if}
						<div class="text-xs text-stone-800 dark:text-stone-300">
							<RelativeDate date={createdAt} />
						</div>
					</div>
				</div>
			</div>
		</div>
	</Card>
</div>
