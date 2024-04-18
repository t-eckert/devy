<script lang="ts">
	import Menu from "$lib/components/menu/menu.svelte"
	import Link from "$lib/components/menu/link.svelte"
	import Sep from "$lib/components/menu/sep.svelte"
	import FormButton from "$lib/components/menu/form-button.svelte"
</script>

<Menu>
	<Link href="/">Home</Link>
	<Sep />
	<Link href="/feedback">Share feedback</Link>
	<Link href="/new/blog">Create your blog</Link>
	<Sep />
	<FormButton action="?/signOut" name="sign out">Sign out</FormButton>
</Menu>
