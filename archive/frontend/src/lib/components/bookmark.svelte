<script lang="ts">
	import Bookmark from "$lib/icons/bookmark.svelte"
	import BookmarkFilled from "$lib/icons/bookmark-filled.svelte"

	import { getBookmarks } from "$lib/state/bookmarks.svelte"
	import { getUser } from "$lib/state/user.svelte"

	const bookmarks = getBookmarks()
	const user = getUser()

	const { postId }: { postId: string } = $props()

	let isBookmarked = $derived(bookmarks.isBookmarked(postId))

	const onclick = async () => {
		await bookmarks.toggle(postId)
	}
</script>

{#if user.isAuthenticated}
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
{/if}
