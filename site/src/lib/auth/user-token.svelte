<script lang="ts">
	import type { Session } from "$lib/types"
	import session from "$lib/auth/session-store"

	let data: Session | undefined = undefined

	session.subscribe((value) => {
		data = value.session
	})
</script>

{#if data}
	<a
		href={"/profiles/" + data.metadata.user.username}
		class="text-sm flex items-center gap-1 rounded-xl justify-center px-2 py-0.5 hover:bg-zinc-100 dark:hover:bg-zinc-800 transition"
	>
		<img
			src={data.metadata.profile.avatarUrl}
			alt="profile"
			class="w-4 aspect-square rounded-full"
		/>
		<span>
			{data.metadata.profile.displayName}
		</span>
	</a>
{:else}
	<a
		href="/api/auth/signin"
		class="text-sm flex items-center rounded-xl justify-center px-2 py-0.5 hover:bg-zinc-100 dark:hover:bg-zinc-800 transition"
	>
		Sign in
	</a>
{/if}
