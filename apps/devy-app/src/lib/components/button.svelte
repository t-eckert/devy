<script lang="ts">
	import type { Snippet } from "svelte"
	import { cva } from "cva"

	type Role = "primary" | "secondary" | "tertiary"
	type Tone = "neutral" | "positive" | "negative"
	type Shape = "rectangle" | "circle"

	interface Props {
		children: Snippet
		role?: Role
		tone?: Tone
		shape?: Shape
		href?: string
		onclick?: () => void
		disabled?: boolean
		name?: string
		value?: string | number | string[] | null | undefined
	}

	const {
		role = "primary",
		tone = "neutral",
		shape = "rectangle",
		href,
		onclick = () => {},
		disabled = false,
		name,
		value,
		children
	}: Props = $props()

	let button = cva(
		"select-none group flex flex-row gap-1 items-center justify-center transition-all disabled:cursor-not-allowed",
		{
			variants: {
				role: {
					primary: "",
					secondary: "",
					tertiary: [
						"bg-none",
						"disabled:text-stone-300",
						"hover:disabled:text-stone-300",
						"hover:disabled:bg-none"
					]
				},
				tone: {
					neutral: "",
					positive: "",
					negative: ""
				},
				shape: {
					rectangle: "rounded px-2 py-1",
					circle: "rounded-full aspect-square p-2.5"
				}
			},
			compoundVariants: [
				{
					role: "primary",
					tone: "neutral",
					class: [
						"bg-stone-800",
						"text-stone-50",
						"dark:bg-zinc-800",
						"dark:text-zinc-300",
						"hover:bg-stone-950"
					]
				},
				{
					role: "primary",
					tone: "positive",
					class: ["bg-emerald-600", "text-emerald-50", "hover:bg-emerald-700"]
				},
				{
					role: "primary",
					tone: "negative",
					class: ["bg-red-600", "text-red-50", "hover:bg-red-700"]
				},
				{
					role: "secondary",
					tone: "neutral",
					class: ["bg-stone-200", "text-stone-800", "hover:text-stone-900", "hover:bg-stone-300"]
				},
				{
					role: "secondary",
					tone: "positive",
					class: [
						"bg-emerald-200",
						"text-emerald-800",
						"hover:text-emerald-900",
						"hover:bg-emerald-300"
					]
				},
				{
					role: "secondary",
					tone: "negative",
					class: ["bg-red-200", "text-red-800", "hover:text-red-900", "hover:bg-red-300"]
				},
				{
					role: "tertiary",
					tone: "neutral",
					class: [
						"text-stone-700",
						"dark:text-zinc-300",
						"hover:bg-stone-100",
						"dark:hover:bg-zinc-900",
						"hover:text-stone-900",
						"dark:hover:text-zinc-300"
					]
				},
				{
					role: "tertiary",
					tone: "positive",
					class: ["text-green-700", "hover:bg-green-100", "hover:text-green-900"]
				},
				{
					role: "tertiary",
					tone: "negative",
					class: ["text-red-700", "hover:bg-red-100", "hover:text-red-900"]
				}
			]
		}
	)
</script>

{#if href}
	<a class={button({ role, tone, shape })} {href}>
		{@render children()}
	</a>
{:else}
	<button class={button({ role, tone, shape })} {onclick} {disabled} {name} {value}>
		{@render children()}
	</button>
{/if}
