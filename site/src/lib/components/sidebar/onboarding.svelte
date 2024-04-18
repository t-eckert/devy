<script lang="ts">
	import type { Session } from "$lib/types"
	import session from "$lib/auth/session-store"
	import Card from "$lib/components/card/card.svelte"

	let userSession: Session | undefined = undefined

	session.subscribe((value) => {
		userSession = value.session
	})
</script>

<Card>
	<div class="p-2">
		<p>To get started with your own blog,</p>
		<ol class="flex flex-col gap-1 py-1 list-decimal font-medium">
			<li class="ml-5">
				<a href="/api/auth/signin">Sign in with GitHub</a>
			</li>

			<li class="ml-5">
				<a href="https://github.com/apps/devy-github">Add Devy to your GitHub</a>
			</li>

			<li class="ml-5">
				<a href="/new/blog" class="font-medium">Connect a Repo</a>
			</li>
		</ol>
	</div>
</Card>
