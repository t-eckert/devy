<script lang="ts">
	export let logs: string[] = []
</script>

<pre
	class="bg-zinc-50/50 px-2 py-1.5 w-96 rounded-md text-zinc-800 text-sm border border-zinc-200"><code
		>{#each logs as log}{log + "\n"}{/each}</code
	></pre>
