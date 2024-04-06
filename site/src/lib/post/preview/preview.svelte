<script lang="ts">
	import type { Post } from "$lib/types"
	import Json from "$lib/utils/json.svelte"
	import { Slash } from "radix-icons-svelte"
	import H2 from "$lib/element/h2.svelte"
	import Like from "$lib/post/like/like.svelte"
	import RelativeDate from "$lib/utils/relative-date.svelte"

	export let post: Post
	const { title, slug, createdAt } = post
</script>

<div class="flex flex-col">
	<div class="font-medium">
		<H2>{title}</H2>
	</div>
	<div class="text-xs"><RelativeDate date={createdAt} /></div>
</div>
