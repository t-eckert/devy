<script lang="ts">
	import { DropdownMenu } from "bits-ui"
	import { fly } from "svelte/transition"

	export let side: "top" | "right" | "bottom" | "left" = "bottom"
	export let align: "start" | "center" | "end" = "end"
</script>

<DropdownMenu.Content
	class="w-48 rounded-md bg-zinc-50/50 text-zinc-800 py-1.5 flex flex-col gap-1 border border-zinc-200 backdrop-blur-lg shadow-sm"
	{side}
	{align}
	sideOffset={4}
	transition={fly}
>
	<slot />
</DropdownMenu.Content>
