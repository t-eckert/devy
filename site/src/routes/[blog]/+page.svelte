<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import H1 from "$lib/elements/h1.svelte"
	import Collection from "$lib/entry/collection/collection.svelte"

	export let data: PageData
</script>

<Main>
	<H1>{data.props.blog.name}</H1>
	<Collection entries={data.props.entries} />
</Main>
