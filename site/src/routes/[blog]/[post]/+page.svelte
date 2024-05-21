<script lang="ts">
	import type { PageData } from "./$types"
	import Markdown from "$lib/markdown/markdown.svelte"
	import Main from "$lib/layouts/main.svelte"
	import RelativeDate from "$lib/utils/relative-date.svelte"
	import H1 from "$lib/elements/h1.svelte"

	export let data: PageData
</script>

<Main>
	<div class="my-8 sm:my-16 mx-auto w-full max-w-3xl flex flex-col">
		<div class="border-t border-zinc-300 w-full">
			<a href={`/${data.props.entry.blogSlug}`} class="text-sm text-stone-500"
				>{data.props.entry.blogName}</a
			>
		</div>
		<h1 class="text-3xl sm:text-5xl font-bold mb-2 break-normal text-zinc-900 sm:leading-tight">
			{data.props.entry.title}
		</h1>
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
