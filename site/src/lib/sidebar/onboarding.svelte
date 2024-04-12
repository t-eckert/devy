<script lang="ts">
	import type { Session } from "$lib/types"
	import session from "$lib/auth/session-store"

	let userSession: Session | undefined = undefined

	session.subscribe((value) => {
		userSession = value.session
	})
</script>

<div>
	<p>To get started with your own blog,</p>
	<ol class="flex flex-col gap-1 py-1 list-decimal font-medium">
		{#if !userSession}
			<li class="ml-5">
				<a href="/api/auth/signin">Sign in with GitHub</a>
			</li>
		{:else}
			<li class="ml-5 line-through text-zinc-400">
				<div class="select-none">Sign in with GitHub</div>
			</li>
		{/if}

		<li class="ml-5">
			<a href="https://github.com/apps/devy-github">Add Devy to your GitHub</a>
		</li>
		<!-- <li class="ml-5 line-through text-zinc-400">
			<div>Add Devy to your GitHub</div>
		</li> -->

		<li class="ml-5">
			<a href="/new/blog" class="font-medium">Connect a Repo</a>
		</li>
	</ol>
	<div></div>
</div>
