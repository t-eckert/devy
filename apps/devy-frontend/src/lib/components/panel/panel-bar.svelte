<script lang="ts">
	import { cva } from "cva"

	type Tone = "neutral" | "positive" | "negative"

	interface Props {
		title: string
		tone?: Tone
	}

	let { title, tone = "neutral" }: Props = $props()

	const bar = cva("w-full px-3 py-0.5 rounded-t border-b", {
		variants: {
			tone: {
				neutral: "bg-stone-50 text-stone-800 dark:text-zinc-200 border-b-stone-100",
				positive: "bg-emerald-200 border-b-emerald-300",
				negative: "bg-red-200 text-red-800 dark:bg-red-600 dark:text-red-200"
			}
		}
	})
</script>

<div class={bar({ tone })}>
	<span class="text-sm font-medium">{title}</span>
</div>
