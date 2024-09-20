<script lang="ts">
	import Sidebar from "$lib/components/sidebar/sidebar.svelte"
	import SidebarSection from "$lib/components/sidebar/sidebar-section.svelte"
	import SidebarLink from "$lib/components/sidebar/sidebar-link.svelte"

	import Home from "$lib/icons/home.svelte"
	import Pencil from "$lib/icons/pencil-1.svelte"
	import Disc from "$lib/icons/disc.svelte"
	import Gear from "$lib/icons/gear.svelte"
</script>

<Sidebar>
	<SidebarSection>
		<SidebarLink href="/dashboard" matchExact>
			<span class="group-hover:rotate-12 transition-all"><Home /></span>
			<span>Overview</span>
		</SidebarLink>
	</SidebarSection>

	<SidebarSection title="Blogs">
		<SidebarLink href="/dashboard/drafts">
			<span class="group-hover:rotate-12 transition-all"><Pencil /></span>
			<span>Drafts</span>
		</SidebarLink>
		<SidebarLink href="/dashboard/manage">
			<span class="group-hover:rotate-12 transition-all"><Disc /></span>
			<span>Manage</span>
		</SidebarLink>
	</SidebarSection>

	<SidebarSection title="Settings">
		<SidebarLink href="/dashboard/profile">
			<span class="group-hover:rotate-12 transition-all"><Gear /></span>
			<span>Profile</span>
		</SidebarLink>
		<SidebarLink href="/dashboard/user-interface">
			<span class="group-hover:rotate-12 transition-all"><Gear /></span>
			<span>User Interface</span>
		</SidebarLink>
		<SidebarLink href="/dashboard/subscription">
			<span class="group-hover:rotate-12 transition-all"><Gear /></span>
			<span>Subscription</span>
		</SidebarLink>
	</SidebarSection>
</Sidebar>
