<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import CallingCard from "$lib/components/calling-card/calling-card.svelte"
	import Preview from "$lib/entry/preview/preview.svelte"
	import Column from "$lib/layouts/column.svelte"
	export let data: PageData

	let { blogs, profile, user, entries } = data.props
</script>

<Main>
	<div class="flex flex-col sm:flex-row gap-8">
		<CallingCard {blogs} {profile} {user} />
		<Column>
			{#each entries as entry}
				<Preview {entry} />
			{/each}
		</Column>
	</div>
</Main>
