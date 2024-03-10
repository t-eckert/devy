<script lang="ts">
	import type { PageData } from "./$types"
	import type { Post } from "$lib/types"
	import Main from "$lib/layout/main.svelte"
	import Collection from "$lib/post/collection/collection.svelte"
	import H1 from "$lib/element/h1.svelte"

	export let data: PageData
	const posts: Post[] = data.props.feeds[0].posts as Post[]
</script>

<Main>
	<div class="grid grid-cols-4">
		<div class="col-start-2 col-span-2">
			<H1>New Posts</H1>
			<Collection {posts} />
		</div>
	</div>
</Main>
