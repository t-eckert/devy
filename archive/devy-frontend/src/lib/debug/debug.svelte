<script lang="ts">
	import Button from "$lib/components/button.svelte"
	import HobbyKnife from "$lib/icons/hobby-knife.svelte"

	import SessionState from "./session-state.svelte"

	let isOpen = $state(false)
</script>

<div class="fixed bottom-12 right-12 z-50">
	<div class="group rounded-full shadow-2xl">
		<Button
			onclick={() => {
				isOpen = !isOpen
			}}
			shape="circle"
		>
			<div class="transition-all group-hover:-rotate-12">
				<HobbyKnife />
			</div>
		</Button>
	</div>
</div>

{#if isOpen}
	<div class="fixed top-0 z-40 flex h-screen w-screen items-center justify-center bg-stone-400/20">
		<div class="rounded bg-white shadow-lg">
			<SessionState />
		</div>
	</div>
{/if}
