<script lang="ts">
	interface Props {
		displayName: string
		src?: string
		rounded?: boolean
	}

	const { src, displayName, rounded = false }: Props = $props()

	let initials = displayName
		.split(" ")
		.map((name) => name[0])
		.join("")
</script>

<div
	class={`w-full h-full grid grid-cols-1 grid-rows-1 bg-stone-100 items-center justify-center ${rounded ? "rounded-full" : "rounded-xl"}`}
>
	{#if src != undefined}
		<img
			{src}
			alt={displayName}
			class={`z-10 col-start-1 col-span-1 ${rounded ? "rounded-full" : "rounded-xl"}`}
		/>
	{:else}
		<div class="font-medium text-stone-800 col-start-1 col-span-1 text-center">
			{initials}
		</div>
	{/if}
</div>
