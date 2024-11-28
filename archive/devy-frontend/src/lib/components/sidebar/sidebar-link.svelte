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
		"group flex flex-row items-center gap-2 rounded-md border px-2 py-1 text-sm font-medium transition-all sm:gap-3",
		isActive
			? "border-stone-200 bg-white text-stone-950 shadow dark:border-zinc-700 dark:bg-zinc-800 dark:text-white dark:shadow-none"
			: "border-transparent text-stone-600 hover:text-stone-950 dark:text-zinc-500"
	].join(" ")}>{@render children()}</a
>
