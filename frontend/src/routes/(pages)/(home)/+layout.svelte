<script lang="ts">
	import type { Snippet } from "svelte"

	import CTA from "./cta.svelte"

	import SidebarMain from "$lib/layouts/sidebar-main.svelte"
	import Sidebar from "./sidebar.svelte"

	const { children }: { children: Snippet } = $props()
</script>

<SidebarMain>
	<div slot="sidebar">
		<Sidebar />
	</div>
	<div class="grid grid-cols-1 sm:grid-cols-3 gap-10">
		<div class="col-span-2">
			{@render children()}
		</div>
		<div class="">
			<CTA />
		</div>
	</div>
</SidebarMain>
