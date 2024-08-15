<script lang="ts">
	import Bookmark from "$lib/icons/bookmark.svelte"
	import BookmarkFilled from "$lib/icons/bookmark-filled.svelte"

	let isBookmarked = $state(false)

	const onclick = () => {
		isBookmarked = !isBookmarked
	}
</script>

<button class="rounded group" {onclick}>
	{#if isBookmarked}
		<div
			class="w-4 aspect-square text-stone-900 group-hover:-translate-y-1 group-hover:text-purple-500 group-hover:scale-110 transition-all"
		>
			<BookmarkFilled />
		</div>
	{:else}
		<div
			class="w-4 aspect-square text-stone-500 group-hover:-translate-y-1 group-hover:text-purple-500 group-hover:scale-110 transition-all"
		>
			<Bookmark />
		</div>
	{/if}
</button>
