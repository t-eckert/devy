<script lang="ts">
	export let feedConfigs: any
	export let selectedFeed: string

	const base =
		"text-sm px-2 py-1 sm:w-52 flex flex-row items-start font-medium rounded-lg border hover:pl-4 transition-all"
	const selected = "bg-zinc-50 text-stone-950"
	const notSelected = "text-stone-700 border-transparent"
	const style = (id: string, selectedId: string): string => {
		return base + (id === selectedId ? ` ${selected}` : ` ${notSelected}`)
	}
</script>

<div class="flex flex-col col-start-1 sm:flex-col items-start gap-2">
	{#each feedConfigs as feedConfig}
		<a href={`/feeds/${feedConfig.id}`} class={style(feedConfig.id, selectedFeed)}
			>{feedConfig.title}</a
		>
	{/each}
</div>
