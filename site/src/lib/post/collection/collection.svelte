<script lang="ts">
	import type { Post } from "$lib/types"
	import Preview from "$lib/post/preview/preview.svelte"

	export let posts: Post[]
</script>

<section class="flex flex-col gap-6 pt-6">
	{#each posts as post}
		<Preview {post} />
	{/each}
</section>
