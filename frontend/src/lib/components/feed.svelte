<script lang="ts">
	import type { Feed } from "$lib/types"
	import EntryList from "$lib/components/entry-list.svelte"
	import ChevronRight from "$lib/icons/chevron-right.svelte"
	import ChevronLeft from "$lib/icons/chevron-left.svelte"

	const { feed }: { feed: Feed } = $props()

	let entries = $derived(feed.entries)

	let prevPage = $state(`/feeds/${feed.config.id}/${feed.page - 1}`)
	let nextPage = $state(`/feeds/${feed.config.id}/${feed.page + 1}`)

	let hasPrevious = $state(feed.page > 1)
	let hasNext = $state(feed.count === feed.pageSize)
</script>

<div class="mx-auto w-full max-w-xl">
	<div class="mb-1 flex flex-row justify-between">
		{#if hasPrevious}
			<a
				href={prevPage}
				class="text-sm font-medium flex flex-row items-center px-1 rounded group text-stone-700 hover:text-stone-950 transition"
			>
				<div class="group-hover:-translate-x-0.5 transition">
					<ChevronLeft />
				</div>
				<span>Prev</span>
			</a>
		{/if}

		<h1 class="text-sm font-medium text-stone-800 transition-all">{feed.config.name}</h1>

		{#if hasNext}
			<a
				href={nextPage}
				class="text-sm font-medium flex flex-row items-center px-1 rounded group text-stone-700 hover:text-stone-950 transition"
			>
				<span> Next </span>
				<div class="group-hover:translate-x-0.5 transition">
					<ChevronRight />
				</div>
			</a>
		{/if}
	</div>
	{#if feed.count === 0}
		No posts found.
	{/if}
	<EntryList {entries} />
	<div class="mt-1 flex flex-row justify-between">
		{#if hasPrevious}
			<a
				href={prevPage}
				class="text-sm font-medium flex flex-row items-center px-1 rounded group text-stone-700 hover:text-stone-950 transition"
			>
				<div class="group-hover:-translate-x-0.5 transition">
					<ChevronLeft />
				</div>
				<span>Prev</span>
			</a>
		{/if}

		{#if hasNext}
			<a
				href={nextPage}
				class="text-sm font-medium flex flex-row items-center px-1 rounded group text-stone-700 hover:text-stone-950 transition"
			>
				<span> Next </span>
				<div class="group-hover:translate-x-0.5 transition">
					<ChevronRight />
				</div>
			</a>
		{/if}
	</div>
</div>
