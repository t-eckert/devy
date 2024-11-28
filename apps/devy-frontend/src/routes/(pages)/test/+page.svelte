<script lang="ts">
	import Json from "$lib/utils/json.svelte"

	let { data } = $props()
</script>

{#each data.result as blog}{/each}
