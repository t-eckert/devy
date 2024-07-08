<script lang="ts">
	import Heart from "$lib/icons/heart.svelte"
	import HeartFilled from "$lib/icons/heart-filled.svelte"

	import Count from "$lib/utils/count.svelte"

	export let likes: number
	export let liked: boolean

	const like = () => {
		liked = !liked
		likes += liked ? 1 : -1
	}
</script>

<button class="mr-2 text-sm flex flex-row items-center justify-start gap-1.5 group" on:click={like}>
	{#if liked}
		<HeartFilled
			class="w-4 aspect-square text-stone-900 group-hover:-translate-y-1.5 transition-all group-hover:text-red-500 group-hover:rotate-12 group-hover:scale-110"
		/>
	{:else}
		<Heart
			class="w-4 aspect-square text-stone-500 group-hover:-translate-y-1 transition-all group-hover:rotate-12 group-hover:scale-110"
		/>
	{/if}
	<span class="w-8 flex flex-row justify-start text-stone-700 font-medium"
		><Count count={likes} /></span
	>
</button>
