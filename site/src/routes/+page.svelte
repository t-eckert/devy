<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import Feeds from "$lib/feed/feeds.svelte"
	import Sidebar from "$lib/components/sidebar/sidebar.svelte"

	export let data: PageData

	let { feeds } = data
</script>

<Main>
	<div class="sm:py-4 grid grid-cols-1 sm:grid-cols-4 gap-4 items-start">
		<div class="col-start-1 sm:col-span-3">
			<Feeds {feeds} />
		</div>
		<div class="hidden sm:block sm:col-span-1">
			<Sidebar />
		</div>
	</div>
</Main>
