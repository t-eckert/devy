<script lang="ts">
	import type { Entry } from "$lib/types"
	import Json from "$lib/utils/json.svelte"
	import { Slash } from "radix-icons-svelte"
	import H2 from "$lib/element/h2.svelte"
	import Like from "$lib/post/like/like.svelte"
	import RelativeDate from "$lib/utils/relative-date.svelte"

	export let entry: Entry
	const { postSlug, title, blogSlug, blogName } = entry
</script>

<div class="flex flex-col">
	<a href={`${blogSlug}`} class="text-xs text-zinc-500">{blogName}</a>
	<a href={`${blogSlug}/${postSlug}`} class="font-medium">{title}</a>
</div>
