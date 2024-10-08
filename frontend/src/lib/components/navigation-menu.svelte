<script lang="ts">
	import { fly } from "svelte/transition"
	import { getUser } from "$lib/state/user.svelte"

	import HamburgerMenu from "$lib/icons/hamburger-menu.svelte"

	type Placement =
		| "top"
		| "top-start"
		| "top-end"
		| "right"
		| "right-start"
		| "right-end"
		| "bottom"
		| "bottom-start"
		| "bottom-end"
		| "left"
		| "left-start"
		| "left-end"

	let positioning = {
		placement: "bottom-end" as Placement
	}

	import { createDropdownMenu, melt } from "@melt-ui/svelte"
	const {
		elements: { menu, item, trigger }
	} = createDropdownMenu({ positioning })

	const itemStyle =
		"text-sm font-medium text-stone-600 dark:text-zinc-300 mx-1 px-2 py-0.5 group flex flex-row rounded items-center pointer-cursor justify-between hover:bg-stone-200/50 dark:hover:bg-zinc-800/70 hover:text-stone-950 dark:hover:text-zinc-100 transition-all"

	const user = getUser()
</script>

<button
	use:melt={$trigger}
	class={[
		"p-1 rounded text-stone-600 dark:text-zinc-300 hover:text-stone-950 dark:hover:text-zinc-100 transition-all border",
		"data-[state=open]:bg-white data-[state=open]:border-stone-200/70 data-[state=open]:shadow data-[state=open]:dark:shadow-none data-[state=open]:dark:bg-zinc-800 data-[state=open]:dark:border-zinc-600/70",
		"data-[state=closed]:bg-none data-[state=closed]:border-transparent data-[state=closed]:hover:bg-stone-100 data-[state=closed]:dark:hover:bg-zinc-800"
	].join(" ")}><HamburgerMenu /></button
>
<div
	use:melt={$menu}
	transition:fly={{ duration: 150, y: -12 }}
	class="w-40 rounded-md border border-stone-200/70 dark:border-zinc-700/70 backdrop-blur-3xl py-1 flex flex-col gap-1 shadow-lg bg-white/80 dark:bg-zinc-900 dark:border-stone-200"
>
	<a use:melt={$item} class={itemStyle} href="/">Home</a>
	<a use:melt={$item} class={itemStyle} href="/changelog">Changelog</a>
	<a use:melt={$item} class={itemStyle} href="/feedback">Feedback</a>
	{#if user.isAuthenticated}
		<a use:melt={$item} class={itemStyle} href="/dashboard">Dashboard</a>
	{/if}
	{#if user.role === "admin"}
		<span class="w-full border-b border-b-stone-200"></span>
		<a use:melt={$item} class={itemStyle} href="/admin">Admin</a>
	{/if}
</div>
