<script lang="ts">
	import type { Session } from "$lib/types"
	import session from "$lib/auth/session-store"
	import { PUBLIC_API } from "$env/static/public"
	let data: Session | undefined = undefined

	session.subscribe((value) => {
		data = value.session
	})
</script>

{#if data}
	<a
		href={"/profiles/" + data.user.username}
		class="text-sm flex items-center gap-1 rounded-xl justify-center px-2 py-0.5 hover:bg-stone-100 dark:hover:bg-stone-800 transition"
	>
		<img src={data.profile.avatarUrl} alt="profile" class="w-4 aspect-square rounded-full" />
		<span>
			{data.profile.displayName}
		</span>
	</a>
{:else}
	<a
		href={`${PUBLIC_API}/auth/login`}
		class="text-sm flex items-center rounded-xl justify-center px-2 py-0.5 hover:bg-stone-100 dark:hover:bg-stone-800 transition"
	>
		Sign in
	</a>
{/if}
