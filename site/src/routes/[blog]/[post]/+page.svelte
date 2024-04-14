<script lang="ts">
	import type { PageData } from "./$types"
	import SvelteMarkdown from "svelte-markdown"
	import Main from "$lib/layouts/main.svelte"
	import H1 from "$lib/elements/h1.svelte"

	export let data: PageData
</script>

<Main>
	<div class="py-4 mx-auto w-full max-w-3xl">
		<a href={`/${data.props.entry.blogSlug}`} class="text-sm text-zinc-500"
			>{data.props.entry.blogName}</a
		>
		<H1>{data.props.entry.title}</H1>
		<a href={`/profiles/${data.props.entry.authorSlug}`} class="text-sm text-zinc-500"
			>{data.props.entry.authorName}</a
		>
	</div>

	<article class="mx-auto w-full max-w-3xl">
		<SvelteMarkdown source={data.props.entry.body} />
	</article>
</Main>
