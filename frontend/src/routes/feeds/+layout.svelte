<script lang="ts">
	import Main from "$lib/layouts/main.svelte"
	import Sidebar from "$lib/components/sidebar/sidebar.svelte"
</script>

<Main>
	<div class="grid grid-cols-2 gap-3 w-full max-w-4xl mx-auto">
		<div class="col-start-1 col-span-2">
			<slot />
		</div>
		<div class="mt-8 col-start-3 col-span-1">
			<Sidebar />
		</div>
	</div>
</Main>
