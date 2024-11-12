<script lang="ts">
	import Header from "./header.svelte"
	import Body from "./body.svelte"

	const { data } = $props()
	const { post } = data
</script>

<svelte:head>
	<title>{post.title}</title>
</svelte:head>

<Header {post} />
<Body {post} />
