<script lang="ts">
	import { onMount } from "svelte"
	import Panel from "$lib/components/panel"
	import { getSessionState } from "$lib/state/session.svelte"

	onMount(() => {
		const sessionState = getSessionState()
		if (sessionState.session) {
			sessionState.signOut()
		}
	})
</script>

<div class="flex items-center justify-center">
	<div class="w-96 mt-[30vh]">
		<Panel>
			<Panel.Header style="success" slot="header"
				><span class="font-medium">Signed out</span></Panel.Header
			>
			<p class="mb-12">You have been successfully signed out.</p>
			<Panel.Footer slot="footer">
				<div class="flex flex-row gap-2 justify-end items-center">
					<a
						class="px-2 py-1 bg-stone-800 rounded text-stone-50 font-medium text-sm hover:bg-stone-950 transition-all"
						href="/">Go home</a
					>
				</div>
			</Panel.Footer>
		</Panel>
	</div>
</div>
