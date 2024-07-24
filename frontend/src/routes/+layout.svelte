<script lang="ts">
	import "../app.css"
	import "@fontsource-variable/inter"
	import "@fontsource/space-mono"
	import Json from "$lib/utils/json.svelte"
	import { setSessionState } from "$lib/state/session.svelte"
	import Session from "$lib/utils/session.svelte"

	let theme = $state("light")
	let { data } = $props()

	const sessionState = setSessionState()
	if (data.token) {
		sessionState.loadToken(data.token)
	}
</script>

<div class={theme}>
	<!-- t-eckert: I don't know how to render the children for a layout -->
	<!-- svelte-ignore slot_element_deprecated -->
	<div class="bg-stone-50 text-stone-950 min-h-screen">
		<slot />
	</div>
</div>
