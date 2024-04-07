<script lang="ts">
	import type { PageData } from "./$types"
	import type { Post } from "$lib/types"
	import Main from "$lib/layout/main.svelte"
	import Feed from "$lib/feed/feed.svelte"
	import Collection from "$lib/post/collection/collection.svelte"
	import H1 from "$lib/element/h1.svelte"
	import Json from "$lib/utils/json.svelte"

	export let data: PageData
	const { feeds } = data.props
	const feed = feeds[0]
</script>

<Main>
	<div class="grid grid-cols-4">
		<div>
			<h1 class="font-semibold">Recent</h1>
		</div>
		<div class="col-start-2 col-span-2">
			<Feed {feed} />
		</div>
	</div>
</Main>
