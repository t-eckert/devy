<script lang="ts">
	import Menu from "$lib/menu/menu.svelte"
	import Items from "$lib/menu/items.svelte"
	import Link from "$lib/menu/link.svelte"
	import Sep from "$lib/menu/sep.svelte"
</script>

<Menu>
	<Items classes="w-40">
		<Link href="/">Home</Link>
		<Link href="/changelog">Changelog</Link>
		<Link href="/about">About</Link>
		<Link href="#">Open an issue</Link>
		<Sep />
		<Link href="/new/blog">Create your blog</Link>
		<Link href="/uploads">Uploads</Link>
		<Sep />
		<Link href="#">Sign out</Link>
	</Items>
</Menu>
