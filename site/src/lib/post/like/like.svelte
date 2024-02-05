<script lang="ts">
	import type { Post } from "$lib/types"
	import Json from "$lib/utils/json.svelte"
	import H2 from "$lib/element/h2.svelte"
	import Like from "$lib/post/like/like.svelte"
	import { Heart } from "radix-icons-svelte"
	import Count from "$lib/utils/count.svelte"

	export let post: Post
	const { likes } = post
</script>

<button><Count count={likes} /></button>
