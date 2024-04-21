<script lang="ts">
	import { HamburgerMenu } from "radix-icons-svelte"
	import { DropdownMenu } from "bits-ui"
	import { fly } from "svelte/transition"

	export let side: "top" | "right" | "bottom" | "left" = "bottom"
	export let align: "start" | "center" | "end" = "end"
	export let classes = ""
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger
		class="p-1 hover:bg-stone-100 rounded-md border border-transparent transition data-[state=open]:bg-stone-50/50 data-[state=open]:border-stone-200 data-[state=open]:shadow-sm"
	>
		<slot name="icon">
			<HamburgerMenu />
		</slot>
	</DropdownMenu.Trigger>

	<DropdownMenu.Content
		class={[
			"rounded-md w-40 bg-stone-50/50 text-stone-800 py-1.5 flex flex-col gap-1 border border-stone-200 backdrop-blur-lg shadow-sm",
			classes
		].join(" ")}
		{side}
		{align}
		sideOffset={4}
		transition={fly}
	>
		<slot />
	</DropdownMenu.Content>
</DropdownMenu.Root>
