<script lang="ts">
	import { HamburgerMenu } from "radix-icons-svelte"
	import { DropdownMenu } from "bits-ui"
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger
		class="p-1 hover:bg-zinc-100 rounded-md border border-transparent transition data-[state=open]:bg-zinc-50/50 data-[state=open]:border-zinc-200 data-[state=open]:shadow-sm"
	>
		<slot name="icon">
			<HamburgerMenu />
		</slot>
	</DropdownMenu.Trigger>
	<slot />
</DropdownMenu.Root>
