<script lang="ts">
	import { getSessionState } from "$lib/state/session.svelte"
	import Wrapper from "./wrapper.svelte"

	const session = getSessionState()
</script>

<Wrapper title="Session State">
	{#if !session.signedIn}
		<span>No user signed in.</span>
	{:else if session.session}
		<div class="flex flex-col items-start">
			<span class="text-sm font-medium text-stone-700">{session.session.username}</span>
			<span class="font-medium mb-1">{session.session.displayName}</span>
			<div class="flex flex-row gap-1">
				<span class="flex border border-stone-300 rounded text-xs">
					<span class="pl-1 pr-1 bg-stone-200 rounded-l-sm">Role</span>
					<span class="pl-1 pr-1 font-mono">{session.session.role}</span>
				</span>
				<span class="flex border border-stone-300 rounded text-xs">
					<span class="pl-1 pr-1 bg-stone-200 rounded-l-sm">Status</span>
					<span class="pl-1 pr-1 font-mono">{session.session.status}</span>
				</span>
			</div>
		</div>
	{/if}
</Wrapper>
