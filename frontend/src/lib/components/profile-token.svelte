<script lang="ts">
	import Avatar from "./avatar.svelte"

	interface Props {
		username: string
		displayName?: string
		avatarUrl?: string
	}

	const { username, displayName, avatarUrl }: Props = $props()
</script>

<a
	class="pl-0.5 pr-2 py-0.5 flex flex-row gap-1 items-center group hover:bg-stone-100 rounded-full transition"
	href={`/profiles/${username}`}
>
	<div class="h-5 w-5 rounded-full">
		<Avatar src={avatarUrl} displayName={displayName || username} rounded={true} />
	</div>
	<div class="text-sm font-medium text-stone-600 hover:text-stone-950">
		{displayName || "ahahaha"}
	</div>
</a>
