<script lang="ts">
	import type { Entry } from "$lib/types"
	import Json from "$lib/utils/json.svelte"
	import { Slash } from "radix-icons-svelte"
	import H2 from "$lib/element/h2.svelte"
	import RelativeDate from "$lib/utils/relative-date.svelte"

	export let entry: Entry
	const { postSlug, title, blogSlug, blogName, authorName, authorSlug, updatedAt, createdAt } =
		entry
</script>

<div class="flex flex-col">
	<a href={`${blogSlug}`} class="text-xs text-zinc-500">{blogName}</a>
	<a href={`${blogSlug}/${postSlug}`} class="font-medium">{title}</a>
	<div class="flex flex-row gap-1">
		<a href={`/profiles/${authorSlug}`} class="text-xs text-zinc-500">{authorName}</a>
		<div class="text-xs"><RelativeDate date={createdAt} /></div>
	</div>
</div>
