<script lang="ts">
	import Json from "$lib/utils/json.svelte"
	import Entry from "$lib/components/entry-preview.svelte"
	import RelativeDate from "$lib/utils/relative-date.svelte"

	const { data } = $props()
	const { drafts } = data
</script>

<svelte:head>
	<title>Dashboard</title>
</svelte:head>

<h1 class="text-2xl font-semibold mb-4">Dashboard</h1>

<h1 class="text-xl font-semibold text-stone-800 mb-1">Drafts</h1>
<section class="flex flex-col lg:flex-row lg:gap-6 items-start w-full">
	<div class="w-96">
		<div class="flex flex-col items-start gap-1 text-sm mb-3">
			<p>
				Draft posts will not be listed in feeds, blogs, or on your profile. They are publically
				visible to anyone with the URL.
			</p>
			<p>
				You can control whether or not your post is in draft mode using the
				<code class="bg-stone-200 text-[0.9em]">draft: true</code>
				frontmatter in your markdown.
			</p>
		</div>
	</div>
	{#if drafts.length === 0}
		<div class="w-full h-40 rounded border border-stone-200 bg-stone-100 shadow-inner">
			<div class="w-full h-full flex items-center justify-center">
				<span class="text-stone-600 font-medium">No drafts</span>
			</div>
		</div>
	{:else}
		<div class="bg-white shadow border border-stone-200 rounded overflow-hidden w-full">
			<table class="w-full divide-y">
				<thead>
					<tr class="bg-stone-100">
						<th class="pl-3 text-left font-medium text-stone-950">Title</th>
						<th class="pr-3 text-right font-medium text-stone-950">Created</th>
					</tr>
				</thead>
				<tbody class="bg-white divide-y">
					{#each drafts as entry}
						<tr class="rounded mb-1">
							<td
								class="text-left pl-3 py-1.5 font-medium text-stone-700 hover:text-stone-950 dark:text-stone-200 transition-all"
							>
								<a
									href={`/${entry.blogSlug}/${entry.postSlug}`}
									class="font-medium text-stone-700 hover:text-stone-950 dark:text-stone-200 transition-all"
									>{entry.title}</a
								>
							</td>
							<td class="text-right pl-2 pr-3 py-1.5 text-stone-800 dark:text-stone-300 text-nowrap"
								><RelativeDate date={entry.createdAt} /></td
							>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</section>
