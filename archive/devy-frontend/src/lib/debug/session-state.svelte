<script lang="ts">
	import { getSession } from "$lib/state/session.svelte"

	let session = getSession()
</script>

<div>{session.isAuthenticated}</div>
