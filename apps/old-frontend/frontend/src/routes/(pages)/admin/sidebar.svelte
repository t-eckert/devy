<script lang="ts">
	import Sidebar from "$lib/components/sidebar/sidebar.svelte"
	import SidebarLink from "$lib/components/sidebar/sidebar-link.svelte"

	import EyeOpen from "$lib/icons/eye-open.svelte"
	import Person from "$lib/icons/person.svelte"
</script>

<Sidebar>
	<SidebarLink href="/admin" matchExact={true}>
		<div class="flex flex-row items-center gap-2">
			<span>
				<EyeOpen />
			</span>
			<span>Overview</span>
		</div>
	</SidebarLink>
	<SidebarLink href="/admin/users">
		<div class="flex flex-row items-center gap-2">
			<span>
				<Person />
			</span>
			<span>Users</span>
		</div>
	</SidebarLink>
</Sidebar>
