<script lang="ts">
	import { page } from "$app/stores"
	import type { Snippet } from "svelte"

	import SidebarMain from "$lib/layouts/sidebar-main.svelte"
	import Sidebar from "$lib/components/sidebar/sidebar.svelte"
	import SidebarLink from "$lib/components/sidebar/sidebar-link.svelte"

	const { children }: { children: Snippet } = $props()

	let path = $derived($page.url.pathname)
</script>

<SidebarMain>
	<div slot="sidebar">
		<Sidebar>
			<SidebarLink href="/admin" isActive={path === "/admin"}>Overview</SidebarLink>
			<SidebarLink href="/admin/users" isActive={path === "/admin/users"}>Users</SidebarLink>
		</Sidebar>
	</div>

	{@render children()}
</SidebarMain>
