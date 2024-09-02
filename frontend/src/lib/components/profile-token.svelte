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
	class="p-0.5 flex flex-row gap-1 items-center group hover:bg-stone-100 dark:hover:bg-zinc-800 rounded-full transition"
	href={`/profiles/${username}`}
>
	<div class="h-5 w-5 rounded-full">
		<Avatar src={avatarUrl} displayName={displayName || username} rounded={true} />
	</div>
</a>
