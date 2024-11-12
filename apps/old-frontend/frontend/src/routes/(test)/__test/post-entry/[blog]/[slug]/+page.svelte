<script lang="ts">
	import type { Entry, Post } from "$lib/types"
	import Json from "$lib/utils/json.svelte"

	let { data } = $props()
	let { entry, post } = data
</script>

<div class="flex flex-row gap-3">
	<div class="max-w-md">
		<h1>Entry</h1>
		<Json data={entry} />
	</div>
	<div class="max-w-md">
		<h1>Post</h1>
		<Json data={post} />
	</div>
</div>
