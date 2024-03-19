<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layout/main.svelte"
	export let data: PageData

	let { profile } = data.props
</script>

<Main>
	<div class="rounded w-44 flex flex-col">
		<img src={profile.avatarUrl} alt={profile.displayName} class="rounded-t" />
		<div class="border-b border-x rounded-b px-0.5 py-1">
			<span class="text-sm font-medium">{profile.displayName}</span>
		</div>
	</div>
</Main>
