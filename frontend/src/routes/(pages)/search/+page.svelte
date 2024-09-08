<script lang="ts">
	import Placeholder from "$lib/components/placeholder.svelte"
	import { PUBLIC_API } from "$env/static/public"

	let query = $state("")
	let result = $state("result")

	async function search() {
		const resp = await fetch(`${PUBLIC_API}/search?query=${query}`)
		result = await resp.text()
	}
</script>

<Placeholder>Search</Placeholder>

<input type="text" class="border border-stone-200 rounded" bind:value={query} oninput={search} />
<div>{result}</div>
