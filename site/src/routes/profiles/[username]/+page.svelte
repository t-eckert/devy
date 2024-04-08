<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layout/main.svelte"
	import Card from "$lib/profile/card/card.svelte"
	export let data: PageData

	let { profile } = data.props
</script>

<Main>
	<Card {profile} />
</Main>
