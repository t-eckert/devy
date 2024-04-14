<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import Feeds from "$lib/feed/feeds.svelte"
	import Sidebar from "$lib/components/sidebar/sidebar.svelte"

	export let data: PageData

	let { feeds } = data.props
</script>

<Main>
	<div class="py-4 grid grid-cols-1 sm:grid-cols-4 gap-8 items-start">
		<div class="col-start-1 sm:col-span-3">
			<Feeds {feeds} />
		</div>
		<div class="">
			<Sidebar />
		</div>
	</div>
</Main>
