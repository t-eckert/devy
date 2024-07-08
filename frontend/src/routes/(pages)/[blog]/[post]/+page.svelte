<script lang="ts">
	import type { PageData } from "./$types"
	import EntryHeader from "$lib/components/entry-header.svelte"
	import EntryBody from "$lib/components/entry-body.svelte"

	export let data: PageData
	const { entry } = data
</script>

<EntryHeader {entry} />
<EntryBody {entry} />
