<script lang="ts">
	import type { Snippet } from "svelte"

	import SidebarMain from "$lib/layouts/sidebar-main.svelte"
	import Sidebar from "./sidebar.svelte"

	const { children }: { children: Snippet } = $props()
</script>

<SidebarMain>
	<div slot="sidebar">
		<h1 class="text-xl font-semibold mb-4">Dashboard</h1>
		<Sidebar />
	</div>
	{@render children()}
</SidebarMain>
