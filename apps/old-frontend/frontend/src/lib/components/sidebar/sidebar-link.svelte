<script lang="ts">
	import { page } from "$app/stores"
	import type { Snippet } from "svelte"

	interface Props {
		href: string
		children: Snippet
		matchExact?: boolean
	}

	const { href, children, matchExact = false }: Props = $props()

	let path = $derived($page.url.pathname)
	let matchSample = $derived(matchExact ? path : path.slice(0, href.length))
	let isActive = $derived(href === matchSample)
</script>

<a
	{href}
	class={[
		"font-medium px-2 py-1 text-sm border rounded-md flex flex-row items-center gap-2 sm:gap-3 group transition-all",
		isActive
			? "text-stone-950 border-stone-200 bg-white shadow dark:shadow-none dark:bg-zinc-800 dark:text-white dark:border-zinc-700"
			: "text-stone-600 border-transparent hover:text-stone-950 dark:text-zinc-500"
	].join(" ")}>{@render children()}</a
>
