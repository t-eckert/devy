<script lang="ts">
	import { getLikesState } from "$lib/state/likes.svelte"
	import Json from "$lib/utils/json.svelte"

	const likesState = getLikesState()
</script>

<Json data={likesState.__debug()} />
