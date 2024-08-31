<script lang="ts">
	import type { Post } from "$lib/types"
	import Markdown from "$lib/markdown/markdown.svelte"

	let { post }: { post: Post } = $props()
</script>

<div class="mx-auto w-full max-w-3xl">
	<Markdown source={post.body} />
</div>
