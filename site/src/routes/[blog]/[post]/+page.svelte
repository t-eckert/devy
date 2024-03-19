<script lang="ts">
	import SvelteMarkdown from "svelte-markdown"
	import type { PageData } from "./$types"
	import Main from "$lib/layout/main.svelte"
	import H1 from "$lib/element/h1.svelte"

	export let data: PageData
</script>

<Main>
	<div class="py-4 mx-auto w-full max-w-3xl">
		<H1>{data.props.post.title}</H1>
		<a href={`/${data.props.post.blogSlug}`}>{data.props.post.blogName}</a>
		<a href={`/profiles/${data.props.post.authorUsername}`}>{data.props.post.authorName}</a>
	</div>

	<article class="mx-auto w-full max-w-3xl">
		<SvelteMarkdown source={data.props.post.content} />
	</article>
</Main>
