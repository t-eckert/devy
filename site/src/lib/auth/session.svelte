<script lang="ts">
	import Json from "$lib/utils/json.svelte"
	import session from "$lib/auth/session-store"
	import type { Session } from "$lib/types"
	import Window from "$lib/utils/window.svelte"

	let data: Session | null

	session.subscribe((value) => (data = value))
</script>

<Window title="Session">
	<Json data={data || {}} />
</Window>
