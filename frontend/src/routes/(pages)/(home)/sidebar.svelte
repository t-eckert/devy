<script lang="ts">
	import { getSessionState } from "$lib/state/session.svelte"

	import Sidebar from "$lib/components/sidebar/sidebar.svelte"
	import SidebarSection from "$lib/components/sidebar/sidebar-section.svelte"
	import SidebarLink from "$lib/components/sidebar/sidebar-link.svelte"

	import Rocket from "$lib/icons/rocket.svelte"
	import Clock from "$lib/icons/clock.svelte"
	import Person from "$lib/icons/person.svelte"
	import Bookmark from "$lib/icons/bookmark.svelte"
	import Heart from "$lib/icons/heart.svelte"

	let signedIn = $derived(getSessionState().signedIn)
</script>

<Sidebar>
	<SidebarSection title="Feeds">
		<SidebarLink href="/feeds/popular">
			<span class="group-hover:rotate-12 group-hover:text-blue-600 transition-all"><Rocket /></span>
			<span>Popular</span>
		</SidebarLink>
		{#if signedIn}
			<SidebarLink href="/feeds/following">
				<span class="group-hover:rotate-12 group-hover:text-yellow-600 transition-all"
					><Person /></span
				>
				<span>Following</span>
			</SidebarLink>
		{/if}
		<SidebarLink href="/feeds/recent">
			<span class="group-hover:rotate-12 group-hover:text-green-600 transition-all"><Clock /></span>
			<span>Recent</span>
		</SidebarLink>
	</SidebarSection>

	{#if signedIn}
		<SidebarSection title="Collections">
			<SidebarLink href="/collections/bookmarked">
				<span class="group-hover:rotate-12 group-hover:text-purple-600 transition-all"
					><Bookmark /></span
				>
				<span>Bookmarked</span>
			</SidebarLink>

			<SidebarLink href="/collections/liked">
				<span class="group-hover:rotate-12 group-hover:text-red-600 transition-all"><Heart /></span>
				<span>Liked</span>
			</SidebarLink>
		</SidebarSection>
	{/if}
</Sidebar>
