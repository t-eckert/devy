<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import CallingCard from "$lib/components/calling-card/calling-card.svelte"
	export let data: PageData

	let { blogs, profile, user } = data.props
</script>

<Main>
	<CallingCard {blogs} {profile} {user} />
</Main>
