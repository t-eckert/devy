<script>
	import CTA from "./cta.svelte"
	import Sidebar from "./sidebar.svelte"
</script>

<div class="grid grid-cols-1 sm:grid-cols-4 gap-2 sm:gap-10 mx-auto">
	<div>
		<Sidebar />
	</div>
	<div class="sm:col-start-2 sm:col-span-2">
		<slot />
	</div>
	<div class="sm:col-start-4">
		<CTA />
	</div>
</div>
