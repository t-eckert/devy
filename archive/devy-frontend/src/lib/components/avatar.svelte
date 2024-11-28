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
	class={`grid h-full w-full grid-cols-1 grid-rows-1 items-center justify-center bg-stone-100 ${rounded ? "rounded-full" : "rounded-xl"}`}
>
	{#if src != undefined}
		<img
			{src}
			alt={displayName}
			class={`z-10 col-span-1 col-start-1 ${rounded ? "rounded-full" : "rounded-xl"}`}
		/>
	{:else}
		<div class="col-span-1 col-start-1 text-center font-medium text-stone-800">
			{initials}
		</div>
	{/if}
</div>
