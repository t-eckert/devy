<script>
	import Sidebar from "./sidebar.svelte"
</script>

<div class="grid grid-cols-1 sm:grid-cols-4 gap-8 mx-auto">
	<div class="sm:col-start-2 sm:col-span-2">
		<slot />
	</div>
	<div class="sm:col-start-4">
		<Sidebar />
	</div>
</div>
