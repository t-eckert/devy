<script lang="ts">
	let { message }: { message: string } = $props()
</script>

<div
	class="flex h-full min-h-96 w-full items-center justify-center rounded bg-stone-200 dark:bg-zinc-800 dark:text-zinc-50"
>
	<p class="select-none text-sm text-stone-700">{message}</p>
</div>
