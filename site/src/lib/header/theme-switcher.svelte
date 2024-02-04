<script lang="ts">
	import { Sun } from "radix-icons-svelte"
	import Menu from "$lib/menu/menu.svelte"
	import Items from "$lib/menu/items.svelte"
	import Link from "$lib/menu/link.svelte"
</script>

<Menu>
	<Sun slot="icon" />
	<Items>
		<Link href="#">Light</Link>
		<Link href="#">Dark</Link>
		<Link href="#">System</Link>
	</Items>
</Menu>
