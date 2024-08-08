<script lang="ts">
	import Heart from "$lib/icons/heart.svelte"
	import HeartFilled from "$lib/icons/heart-filled.svelte"
	import { getLikesState } from "$lib/state/likes.svelte"

	const likesState = getLikesState()

	const { postId, likes }: { postId: string; likes: number } = $props()

	likesState.setCount(postId, likes)
	let count = $derived(likesState.getCount(postId))
	let isLiked = $derived(likesState.isLiked(postId))

	const onclick = async () => {
		await likesState.toggle(postId)
	}
</script>

<button class="mr-2 text-sm flex flex-row items-center justify-start gap-1.5 group" {onclick}>
	{#if isLiked}
		<div
			class="w-4 aspect-square text-stone-900 group-hover:-translate-y-1.5 transition-all group-hover:text-red-500 group-hover:rotate-12 group-hover:scale-110"
		>
			<HeartFilled />
		</div>
	{:else}
		<div
			class="w-4 aspect-square text-stone-500 group-hover:-translate-y-1 transition-all group-hover:rotate-12 group-hover:scale-110"
		>
			<Heart />
		</div>
	{/if}

	<span class="w-8 flex flex-row justify-start text-stone-700 font-medium">{count}</span>
</button>
