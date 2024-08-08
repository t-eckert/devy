<script lang="ts">
	import { getSessionState } from "$lib/state/session.svelte"
	import Json from "$lib/utils/json.svelte"

	const session = getSessionState()
</script>

<Json data={session.__debug()} />
