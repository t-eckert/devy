<script lang="ts">
	export let logs: string[] = []
</script>

<pre
	class="bg-stone-50/50 px-2 py-1.5 w-96 rounded-md text-stone-800 text-sm border border-stone-200"><code
		>{#each logs as log}{log + "\n"}{/each}</code
	></pre>
