<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layout/main.svelte"
	import CallingCard from "$lib/calling-card/calling-card.svelte"
	export let data: PageData

	let { profile } = data.props
</script>

<Main>
	<CallingCard {profile} />
</Main>
