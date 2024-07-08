<script lang="ts">
	const { src, displayName }: { src?: string; displayName: string } = $props()

	let initials = displayName
		.split(" ")
		.map((name) => name[0])
		.join("")
</script>

<div class="w-full h-full grid grid-cols-1 grid-rows-1 bg-stone-100">
	{#if src != undefined}
		<img {src} alt={displayName} class="z-10 col-start-1 col-span-1" />
	{/if}
	<div class="font-medium text-stone-800 col-start-1 col-span-1">{initials}</div>
</div>
