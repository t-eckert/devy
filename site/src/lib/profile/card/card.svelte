<script lang="ts">
	import type { Profile } from "$lib/types"

	export let profile: Profile
</script>

<div class="rounded w-44 flex flex-col">
	<img src={profile.avatarUrl} alt={profile.displayName} class="rounded-t" />
	<div class="border-b border-x rounded-b px-1 py-1">
		<span class="text-sm font-medium">{profile.displayName}</span>
	</div>
</div>
