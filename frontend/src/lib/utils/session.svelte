<script lang="ts">
	import { getSessionState } from "$lib/state/session.svelte.ts"
	import Window from "$lib/utils/window.svelte"
	import Json from "$lib/utils/json.svelte"

	const session = getSessionState()
</script>

<Window>
	<Json data={session.session} />
</Window>
