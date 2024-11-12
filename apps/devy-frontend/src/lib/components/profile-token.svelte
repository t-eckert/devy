<script lang="ts">
	import Avatar from "$lib/components/avatar.svelte"

	interface Props {
		username: string
		displayName?: string
		avatarUrl?: string
	}

	const { username, displayName, avatarUrl }: Props = $props()
</script>

<a
	class="group flex flex-row items-center gap-1 rounded-full py-0.5 pl-0.5 pr-2 transition hover:bg-stone-100 dark:hover:bg-zinc-800"
	href={`/profiles/${username}`}
>
	<div class="h-5 w-5 rounded-full">
		<Avatar src={avatarUrl} displayName={displayName || username} rounded={true} />
	</div>
	<div
		class="sr-only text-sm font-medium text-stone-600 hover:text-stone-950 sm:not-sr-only dark:text-zinc-200 dark:hover:text-zinc-100"
	>
		{displayName || username}
	</div>
</a>
