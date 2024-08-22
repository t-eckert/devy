<script lang="ts">
	const { title }: { title: string } = $props()
</script>

<section
	class="border border-stone-200 rounded-lg w-full h-full bg-white/70 dark:bg-zinc-800/70 dark:border-zinc-800"
>
	<div
		class="px-2 pt-1 pb-0.5 font-medium bg-stone-50 rounded-t-md dark:bg-zinc-900 dark:text-zinc-100"
	>
		<h1 class="text-xs">{title}</h1>
	</div>
	<div class="px-2 py-2">
		<slot />
	</div>
</section>
