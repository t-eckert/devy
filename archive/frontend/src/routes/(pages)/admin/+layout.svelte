<script lang="ts">
	import type { Snippet } from "svelte"

	import SidebarMain from "$lib/layouts/sidebar-main.svelte"
	import Sidebar from "./sidebar.svelte"
	const { children }: { children: Snippet } = $props()
</script>

<SidebarMain>
	<div slot="sidebar">
		<Sidebar />
	</div>

	{@render children()}
</SidebarMain>
