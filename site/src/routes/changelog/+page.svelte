<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layout/main.svelte"
	import H1 from "$lib/element/h1.svelte"
	import Json from "$lib/utils/json.svelte"
	import Markdown from "$lib/markdown/markdown.svelte"

	export let data: PageData
</script>

<Main>
	<H1>Changelog</H1>
	<Markdown source={data.props.body} />
	<Json {data} />
</Main>
