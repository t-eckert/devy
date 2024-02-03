<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layout/main.svelte"
	import Json from "$lib/utils/json.svelte"

	export let data: PageData
</script>

<Main>
	<Json {data} />
</Main>
