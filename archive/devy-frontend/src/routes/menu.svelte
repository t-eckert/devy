<script lang="ts">
	import { cva } from "cva"
	import { createDropdownMenu, melt } from "@melt-ui/svelte"
	import HamburgerMenu from "$lib/icons/hamburger-menu.svelte"

	const {
		elements: { menu, item, trigger }
	} = createDropdownMenu({ positioning: { placement: "bottom-end" } })

	let button = cva([
		"rounded",
		"border",
		"p-1",
		"text-stone-600",
		"transition-all",
		"hover:text-stone-950",
		"dark:text-zinc-300",
		"dark:hover:text-zinc-100",
		"data-[state=open]:border-stone-200/70",
		"data-[state=open]:bg-white",
		"data-[state=open]:shadow",
		"data-[state=open]:dark:border-zinc-600/70",
		"data-[state=open]:dark:bg-zinc-800",
		"data-[state=open]:dark:shadow-none",
		"data-[state=closed]:border-transparent",
		"data-[state=closed]:bg-none",
		"data-[state=closed]:hover:bg-stone-200",
		"data-[state=closed]:dark:hover:bg-zinc-800"
	])
	let section = cva([
		"w-56",
		"sm:w-40",
		"rounded-md",
		"border",
		"border-stone-200",
		"bg-white",
		"dark:border-zinc-700",
		"dark:bg-zinc-700",
		"py-1",
		"flex",
		"flex-col",
		"gap-1",
		"shadow-lg",
		"dark:shadow-none"
	])
	let menuItem = cva([
		"sm:text-sm",
		"font-medium",
		"text-stone-600",
		"dark:text-zinc-300",
		"mx-1",
		"px-2",
		"py-1",
		"sm:py-0.5",
		"group",
		"flex",
		"flex-row",
		"rounded",
		"items-center",
		"pointer-cursor",
		"justify-between",
		"hover:bg-stone-200/50",
		"dark:hover:bg-zinc-800/70",
		"hover:text-stone-950",
		"dark:hover:text-zinc-100",
		"transition-all"
	])
</script>

{#snippet divider()}
	<span class="w-full border-b border-b-stone-200"></span>
{/snippet}

<button use:melt={$trigger} class={button()}><HamburgerMenu /></button>

<section use:melt={$menu} class={section()}>
	<a href="/changelog" use:melt={$item} class={menuItem()}>Changelog</a>
	<a href="/docs" use:melt={$item} class={menuItem()}>Documentation</a>
	<a href="/feedback" use:melt={$item} class={menuItem()}>Feedback</a>
</section>
