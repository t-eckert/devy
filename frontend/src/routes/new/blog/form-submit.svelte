<script lang="ts">
	import { ArrowUp } from "radix-icons-svelte"
	import Button from "$lib/components/button/button.svelte"
</script>

<div class="flex justify-end items-center gap-2">
	<Button behavior="positive">
		<ArrowUp class="w-4 h-4 group-hover:-translate-y-0.5 transition" />
		<span class="text-sm font-medium">Submit</span>
	</Button>
	<a href="/" class="flex flex-row group items-center gap-1 font-medium rounded px-2 py-1 text-sm"
		>Cancel</a
	>
</div>
