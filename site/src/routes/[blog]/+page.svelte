<script lang="ts">
	import Main from "$lib/layout/main.svelte"
	import H1 from "$lib/element/h1.svelte"
	import Collection from "$lib/post/collection/collection.svelte"

	export let data
</script>

<Main>
	<H1>{data.props.blog.name}</H1>
	<Collection posts={data.props.posts} />
</Main>
