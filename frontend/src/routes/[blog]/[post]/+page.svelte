<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import EntryHeader from "$lib/components/entry-header/entry-header.svelte"
	import EntryBody from "$lib/components/entry-body/entry-body.svelte"

	export let data: PageData
</script>

<Main>
	<EntryHeader entry={data.entry} />
	<EntryBody entry={data.entry} />
</Main>
