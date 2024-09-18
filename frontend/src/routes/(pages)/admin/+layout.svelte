<script lang="ts">
	import SidebarMain from "$lib/layouts/sidebar-main.svelte"
</script>

<SidebarMain>
	<div slot="sidebar">
		<a href="/admin/users">Users</a>
	</div>
	<slot />
</SidebarMain>
