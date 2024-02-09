<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layout/main.svelte"
	import H1 from "$lib/element/h1.svelte"
	import Json from "$lib/utils/json.svelte"

	export let data: PageData
</script>

<Main>
	<article>{data.props.post.content}</article>
</Main>
