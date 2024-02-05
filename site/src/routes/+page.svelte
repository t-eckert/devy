<script lang="ts">
	import type { PageData } from "./$types"
	import type { Post } from "$lib/types"
	import Main from "$lib/layout/main.svelte"
	import Collection from "$lib/post/collection/collection.svelte"
	import Json from "$lib/utils/json.svelte"

	export let data: PageData
	const posts: Post[] = data.props.feeds[0].posts as Post[]
</script>

<Main>
	<Collection {posts} />
</Main>
