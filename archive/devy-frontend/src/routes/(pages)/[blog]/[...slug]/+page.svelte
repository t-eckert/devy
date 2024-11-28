<script lang="ts">
	import PostHeader from "./post-header.svelte"
	import Markdown from "$lib/markdown/markdown.svelte"

	const { data } = $props()
	const { post } = data
</script>

<PostHeader {post} />

<div class="mx-auto w-full max-w-3xl">
	<Markdown source={post.body} />
</div>
