<script>
	import Sidebar from "./sidebar.svelte"
</script>

<div class="flex flex-row gap-3 mx-auto">
	<slot />
	<Sidebar />
</div>
