<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layout/main.svelte"
	import H1 from "$lib/element/h1.svelte"
	import Collection from "$lib/entry/collection/collection.svelte"
	import Json from "$lib/utils/json.svelte"

	export let data: PageData
</script>

<Main>
	<H1>{data.props.blog.name}</H1>
	<Collection entries={data.props.entries} />
</Main>
