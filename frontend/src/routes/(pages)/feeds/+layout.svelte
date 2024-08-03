<script>
	import Sidebar from "./sidebar.svelte"
</script>

<div class="grid grid-cols-4 gap-8 mx-auto">
	<div class="col-start-2 col-span-2">
		<slot />
	</div>
	<div class="col-start-4">
		<Sidebar />
	</div>
</div>
