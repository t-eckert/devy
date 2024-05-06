<script lang="ts">
	import type { PageData } from "./$types"
	import Markdown from "$lib/markdown/markdown.svelte"
	import Main from "$lib/layouts/main.svelte"
	import RelativeDate from "$lib/utils/relative-date.svelte"
	import H1 from "$lib/elements/h1.svelte"

	export let data: PageData
</script>

<Main>
	<div class="py-8 mx-auto w-full max-w-3xl flex flex-col gap-2">
		<a href={`/${data.props.entry.blogSlug}`} class="text-sm text-stone-500"
			>{data.props.entry.blogName}</a
		>
		<H1>{data.props.entry.title}</H1>
		<div class="flex flex-row gap-2">
			<a href={`/profiles/${data.props.entry.authorSlug}`} class="text-sm text-stone-500"
				>{data.props.entry.authorName}</a
			>
			<div class="text-sm dark:text-stone-300">
				<RelativeDate date={data.props.entry.createdAt} />
			</div>
		</div>
	</div>

	<article class="mx-auto w-full max-w-3xl">
		<Markdown source={data.props.entry.body} />
	</article>
</Main>
