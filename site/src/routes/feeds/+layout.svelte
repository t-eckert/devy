<script lang="ts">
	import Main from "$lib/layouts/main.svelte"
	import Sidebar from "$lib/components/sidebar/sidebar.svelte"
	import FeedSelector from "$lib/components/feed-selector/feed-selector.svelte"

	const feedConfigs = [
		{
			id: "recent",
			title: "Recent"
		}
	]
</script>

<Main>
	<div class="sm:py-4 grid grid-cols-1 sm:grid-cols-4 gap-6 items-start">
		<div class="col-start-1 sm:col-span-3">
			<div class="flex flex-col sm:flex-row gap-6">
				<FeedSelector {feedConfigs} />
				<slot />
			</div>
		</div>
		<div class="hidden sm:block sm:col-span-1">
			<Sidebar />
		</div>
	</div>
</Main>
