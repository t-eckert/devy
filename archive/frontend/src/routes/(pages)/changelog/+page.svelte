<script lang="ts">
	import H1 from "$lib/elements/h1.svelte"
	import P from "$lib/elements/p.svelte"
	import Markdown from "$lib/markdown/markdown.svelte"

	const { data } = $props()
	const { changelogs } = data
</script>

<svelte:head>
	<title>Changelog</title>
</svelte:head>

<div class="mt-8 mb-6">
	<H1>Changelog</H1>
	<div class="max-w-sm mt-4">
		<P>
			While Devy is released continuously, these changelog entries tag notable changes to the
			project.
		</P>
	</div>
</div>

<div class="flex flex-col gap-6 w-full mx-auto">
	{#each changelogs as { version, body }}
		<div class="grid grid-cols-1 md:grid-cols-4 border-t border-dashed border-stone-300">
			<div class="text-2xl pt-5">
				<code>{version}</code>
			</div>
			<div class="col-span-3">
				<section class="h-full w-full md:px-4 py-1">
					<Markdown source={body} />
				</section>
			</div>
		</div>
	{/each}
</div>
