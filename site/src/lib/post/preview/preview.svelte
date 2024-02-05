<script lang="ts">
	import type { Post } from "$lib/types"
	import Json from "$lib/utils/json.svelte"

	export let post: Post
</script>

<Json data={post} />
