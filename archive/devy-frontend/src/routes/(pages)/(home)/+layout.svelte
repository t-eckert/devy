<script lang="ts">
	import type { Snippet } from "svelte"

	import SidebarContent from "$lib/layouts/sidebar-content.svelte"
	import CTA from "./cta.svelte"
	import HomeSidebar from "./home-sidebar.svelte"

	const { children }: { children: Snippet } = $props()
</script>

<div class="mb-4">
	<SidebarContent>
		{#snippet sidebar()}
			<HomeSidebar />
		{/snippet}

		<div class="grid grid-cols-1 gap-10 sm:grid-cols-3">
			<div class="col-span-2">
				{@render children()}
			</div>
			<div class="hidden sm:block">
				<CTA />
			</div>
		</div>
	</SidebarContent>
</div>
