<script lang="ts">
	import type { Session } from "$lib/types"
	import session from "$lib/auth/session-store"

	let data: Session | null

	session.subscribe((value) => (data = value.session))
</script>

{#if data !== null}
	<div
		class="text-sm flex items-center gap-1 rounded-xl justify-center px-2 py-0.5 hover:bg-zinc-100 dark:hover:bg-zinc-800 transition"
	>
		<img
			src={data.metadata.profile.avatarUrl}
			alt="profile"
			class="w-6 aspect-square rounded-full"
		/>
		<a href={"/profiles/" + data.metadata.user.username}>
			{data.metadata.profile.displayName}
		</a>
	</div>
{:else}
	<a
		href="/api/auth/signin"
		class="text-sm flex items-center rounded-xl justify-center px-2 py-0.5 hover:bg-zinc-100 dark:hover:bg-zinc-800 transition"
	>
		Sign in
	</a>
{/if}
