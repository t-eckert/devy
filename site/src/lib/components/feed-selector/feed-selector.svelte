<script lang="ts">
	export let feedConfigs: any
</script>

<div class="flex flex-col col-start-1 sm:flex-col items-start gap-2">
	{#each feedConfigs as feedConfig}
		<a
			href={`/feeds/${feedConfig.id}`}
			class={[
				"bg-stone-50 text-sm  px-2 py-1 sm:w-52 flex flex-row items-start font-medium rounded-lg border hover:pl-4 transition-all",
				"text-stone-900 border-stone-200 shadow-xs"
			].join(" ")}>{feedConfig.title}</a
		>
	{/each}
</div>
