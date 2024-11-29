<script lang="ts">
	let { data }: { data: object } = $props();
</script>

<pre
	class="max-h-96 max-w-6xl overflow-auto rounded bg-stone-100 p-2 text-sm dark:bg-stone-800"><code
		>{JSON.stringify(data, null, 2)}</code
	></pre>
