<script lang="ts">
	import Bookmark from "$lib/icons/bookmark.svelte"
	import BookmarkFilled from "$lib/icons/bookmark-filled.svelte"

	import { getBookmarks } from "$lib/state/bookmarks.svelte"
	import { getSession } from "$lib/state/session.svelte"

	const bookmarks = getBookmarks()
	const session = getSession()

	const { postId }: { postId: string } = $props()

	let isBookmarked = $derived(bookmarks.isBookmarked(postId))

	const onclick = async () => {
		await bookmarks.toggle(postId)
	}
</script>

{#if session.isAuthenticated}
	<button class="group rounded" {onclick}>
		{#if isBookmarked}
			<div
				class="aspect-square w-4 text-stone-900 transition-all group-hover:-translate-y-1 group-hover:scale-110 group-hover:text-purple-500"
			>
				<BookmarkFilled />
			</div>
		{:else}
			<div
				class="aspect-square w-4 text-stone-500 transition-all group-hover:-translate-y-1 group-hover:scale-110 group-hover:text-purple-500"
			>
				<Bookmark />
			</div>
		{/if}
	</button>
{/if}
