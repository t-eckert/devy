<script lang="ts">
	import Json from "$lib/components/json.svelte";
	import { page } from "$app/stores";

	let props = $props();
</script>

<Json data={$page} />
