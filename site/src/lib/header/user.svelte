<script lang="ts">
	import type { User } from "$lib/types"
	import Token from "$lib/profile/token.svelte"
	import SignInToken from "$lib/auth/sign-in-token.svelte"

	let user: User | null = null
</script>

{#if user}
	<Token {user} />
{:else}
	<SignInToken />
{/if}
