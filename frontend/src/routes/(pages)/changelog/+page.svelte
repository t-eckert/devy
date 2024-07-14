<script lang="ts">
	import type { PageData } from "./$types"
	import H1 from "$lib/elements/h1.svelte"
	import P from "$lib/elements/p.svelte"
	import Card from "$lib/components/card.svelte"
	import Markdown from "$lib/markdown/markdown.svelte"

	export let data: PageData
</script>

<svelte:head>
	<title>Changelog</title>
</svelte:head>

<div class="flex flex-col gap-6 w-full max-w-3xl mx-auto">
	<div class="mt-8 flex flex-col gap-6">
		<H1>Changelog</H1>
		<div class="max-w-sm">
			<P>
				While Devy is released continuously, these changelog entries tag notable changes to the
				project.
			</P>
		</div>
	</div>
	{#each data.props.changelogs as changelog}
		<Card>
			<Markdown source={changelog.markdown} />
		</Card>
	{/each}
</div>
