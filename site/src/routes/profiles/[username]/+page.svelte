<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layout/main.svelte"
	import CallingCard from "$lib/components/calling-card/calling-card.svelte"
	export let data: PageData

	let { blogs, profile } = data.props
</script>

<Main>
	<CallingCard {blogs} {profile} />
</Main>
