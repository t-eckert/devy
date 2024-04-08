<script lang="ts">
	import Menu from "$lib/menu/menu.svelte"
	import Items from "$lib/menu/items.svelte"
	import Link from "$lib/menu/link.svelte"
	import Sep from "$lib/menu/sep.svelte"
	import FormButton from "$lib/menu/form-button.svelte"
</script>

<Menu>
	<Items classes="w-40">
		<Link href="/">Home</Link>
		<Sep />
		<Link href="/feedback">Share your feedback</Link>
		<Link href="/new/blog">Create your blog</Link>
		<Sep />
		<FormButton action="?/signOut" name="sign out">Sign out</FormButton>
	</Items>
</Menu>
