<script lang="ts">
	import Clipboard from "$lib/icons/clipboard.svelte"

	let { text, label }: { text: string; label?: string } = $props()

	const onclick = async () => {
		await navigator.clipboard.writeText(text)
	}
</script>

<div class="flex flex-col items-left">
	{#if label}
		<span class="font-semibold text-xs">{label}</span>
	{/if}
	<div
		class="w-full flex flex-row items-center bg-stone-100 text-stone-950 pl-1 pr-0.5 py-0.5 rounded border border-stone-200"
	>
		<code class="flex-1">{text}</code>
		<button class="hover:bg-stone-200 p-1 rounded-md transition" {onclick}>
			<Clipboard />
		</button>
	</div>
</div>
