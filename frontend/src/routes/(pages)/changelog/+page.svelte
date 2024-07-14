<script lang="ts">
	import type { PageData } from "./$types"
	import H1 from "$lib/elements/h1.svelte"
	import Card from "$lib/components/card.svelte"
	import Markdown from "$lib/markdown/markdown.svelte"

	export let data: PageData
</script>

<svelte:head>
	<title>Changelog</title>
</svelte:head>

<div class="flex flex-col gap-4 w-full max-w-3xl mx-auto">
	<div class="mb-4 mt-8">
		<H1>Changelog</H1>
	</div>
	{#each data.props.changelogs as changelog}
		<Card>
			<Markdown source={changelog.markdown} />
		</Card>
	{/each}
</div>
