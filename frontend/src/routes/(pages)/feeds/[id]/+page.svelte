<script lang="ts">
	import type { PageData } from "./$types"
	import Feed from "$lib/components/feed"

	export let data: PageData
	const { feed } = data.props
</script>

<Feed {feed} />
