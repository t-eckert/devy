<script lang="ts">
	import Json from "$lib/utils/json.svelte"
	import session from "$lib/state/session-store"
	import type { SessionStore } from "$lib/state/session-store"
	import Window from "$lib/utils/window.svelte"

	let data: SessionStore

	session.subscribe((value) => (data = value))
</script>

<Window title="Session Store">
	<Json {data} />
</Window>
