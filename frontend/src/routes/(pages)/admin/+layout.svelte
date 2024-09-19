<script lang="ts">
	import { page } from "$app/stores"
	import type { Snippet } from "svelte"

	import SidebarMain from "$lib/layouts/sidebar-main.svelte"
	import Sidebar from "$lib/components/sidebar/sidebar.svelte"
	import SidebarLink from "$lib/components/sidebar/sidebar-link.svelte"

	import EyeOpen from "$lib/icons/eye-open.svelte"
	import Person from "$lib/icons/person.svelte"

	const { children }: { children: Snippet } = $props()

	let path = $derived($page.url.pathname)
</script>

<SidebarMain>
	<div slot="sidebar">
		<Sidebar>
			<SidebarLink href="/admin" isActive={path === "/admin"}>
				<div class="flex flex-row items-center gap-2">
					<span>
						<EyeOpen />
					</span>
					<span>Overview</span>
				</div>
			</SidebarLink>
			<SidebarLink href="/admin/users" isActive={path === "/admin/users"}>
				<div class="flex flex-row items-center gap-2">
					<span>
						<Person />
					</span>
					<span>Users</span>
				</div>
			</SidebarLink>
		</Sidebar>
	</div>

	{@render children()}
</SidebarMain>
