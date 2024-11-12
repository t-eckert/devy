<script lang="ts">
	import type { Snippet } from "svelte"

	type Role = "primary" | "secondary" | "tertiary"
	type Behavior = "neutral" | "positive" | "negative"

	interface Props {
		children: Snippet
		role?: Role
		behavior?: Behavior
		href?: string
		onclick?: () => void
		disabled?: boolean
		name?: string
		value?: string | number | string[] | null | undefined
	}

	const {
		role = "primary",
		behavior = "neutral",
		href,
		onclick = () => {},
		disabled = false,
		name,
		value,
		children
	}: Props = $props()

	let rootStyles =
		"select-none group flex flex-row rounded-md gap-1 items-center px-2 py-1 transition-all disabled:cursor-not-allowed"
	let style = {
		primary: {
			neutral: "bg-stone-800 text-stone-50 hover:bg-stone-900",
			positive: "bg-emerald-600 text-emerald-50 hover:bg-emerald-700",
			negative: "bg-red-600 text-red-50 hover:bg-red-700"
		},
		secondary: {
			neutral: "bg-stone-200 text-stone-800 hover:text-stone-900 hover:bg-stone-300",
			positive: "bg-emerald-200 text-emerald-800 hover:text-emerald-900 hover:bg-emerald-300",
			negative: "bg-red-200 text-red-800 hover:text-red-900 hover:bg-red-300"
		},
		tertiary: {
			neutral:
				"bg-none text-stone-700 hover:bg-stone-100 hover:text-stone-900 disabled:text-stone-300 hover:disabled:text-stone-300 hover:disabled:bg-none",
			positive: "bg-none text-green-700 hover:bg-green-100 hover:text-green-900",
			negative: "bg-none text-red-700 hover:bg-red-100 hover:text-red-900"
		}
	}[role][behavior]
</script>

{#if href}
	<a class={`${style} ${rootStyles}`} {href}>
		{@render children()}
	</a>
{:else}
	<button class={`${style} ${rootStyles}`} {onclick} {disabled} {name} {value}>
		{@render children()}
	</button>
{/if}
