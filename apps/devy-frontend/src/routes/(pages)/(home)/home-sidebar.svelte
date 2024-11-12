<script lang="ts">
	import { getSession } from "$lib/state/session.svelte"

	import Sidebar from "$lib/components/sidebar/sidebar.svelte"
	import SidebarSection from "$lib/components/sidebar/sidebar-section.svelte"
	import SidebarLink from "$lib/components/sidebar/sidebar-link.svelte"
	import Button from "$lib/components/button.svelte"

	import Rocket from "$lib/icons/rocket.svelte"
	import Clock from "$lib/icons/clock.svelte"
	import Person from "$lib/icons/person.svelte"
	import Bookmark from "$lib/icons/bookmark.svelte"
	import Heart from "$lib/icons/heart.svelte"
	import PaperPlane from "$lib/icons/paper-plane.svelte"

	const session = getSession()
</script>

<Sidebar>
	<SidebarSection title="Feeds">
		<SidebarLink href="/feeds/popular">
			<span class="transition-all group-hover:rotate-12 group-hover:text-blue-600"><Rocket /></span>
			<span>Popular</span>
		</SidebarLink>
		{#if session.isAuthenticated}
			<SidebarLink href="/feeds/following">
				<span class="transition-all group-hover:rotate-12 group-hover:text-yellow-600"
					><Person /></span
				>
				<span>Following</span>
			</SidebarLink>
		{/if}
		<SidebarLink href="/feeds/recent">
			<span class="transition-all group-hover:rotate-12 group-hover:text-green-600"><Clock /></span>
			<span>Recent</span>
		</SidebarLink>
	</SidebarSection>

	{#if session.isAuthenticated}
		<SidebarSection title="Collections">
			<SidebarLink href="/collections/bookmarked">
				<span class="transition-all group-hover:rotate-12 group-hover:text-purple-600"
					><Bookmark /></span
				>
				<span>Bookmarked</span>
			</SidebarLink>

			<SidebarLink href="/collections/liked">
				<span class="transition-all group-hover:rotate-12 group-hover:text-red-600"><Heart /></span>
				<span>Liked</span>
			</SidebarLink>
		</SidebarSection>
	{/if}

	<div class="hidden sm:contents">
		<section class="mt-3 flex flex-col items-start">
			<Button role="secondary" href="/feedback">
				<div class="flex flex-row items-center gap-1">
					<span class="-rotate-45"><PaperPlane /></span>
					<span class="text-sm font-medium">Share your feedback</span>
				</div>
			</Button>
		</section>
	</div>
</Sidebar>
