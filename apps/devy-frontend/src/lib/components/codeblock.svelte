<script lang="ts">
	import Button from "$lib/components/button.svelte"
	import Clipboard from "$lib/icons/clipboard.svelte"
	import { codeToHtml } from "shiki"
	import { onMount } from "svelte"

	let { lang, code }: { lang: string; code: string } = $props()

	let html = $state("")
	onMount(async () => {
		html = await codeToHtml(code, { theme: "catppuccin-frappe", lang })
	})

	const onclick = async () => {
		await navigator.clipboard.writeText(code)
	}
</script>

<div class="relative">
	<div class="z-10 absolute top-1 right-1">
		<Button role="secondary" {onclick}><Clipboard /></Button>
	</div>
	{@html html}
</div>
