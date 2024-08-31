<script lang="ts">
	import Wrapper from "./wrapper.svelte"
	import CopyableField from "$lib/components/copyable-field.svelte"

	import { getUser } from "$lib/state/user.svelte"

	const user = getUser()
</script>

<Wrapper title="User">
	<div class="w-full flex flex-row justify-between items-center dark:text-white">
		{#if user.isAuthenticated}
			<div class="flex flex-col gap-2 w-full">
				<CopyableField label="Display Name" text={user.displayName || ""} />
				<CopyableField label="User ID" text={user.userId || ""} />
				<CopyableField label="Profile ID" text={user.profileId || ""} />
				<CopyableField label="Username" text={user.username || ""} />
				<CopyableField label="Role" text={user.role || ""} />
				<CopyableField label="Status" text={user.status || ""} />
			</div>
		{:else}
			No user logged in.
		{/if}
	</div>
</Wrapper>
