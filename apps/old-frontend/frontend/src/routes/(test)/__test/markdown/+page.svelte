<script lang="ts">
	import Markdown from "$lib/markdown/markdown.svelte"
	import parser from "$lib/markdown/parser"

	import { defaultText } from "./default-text"

	let source = $state(defaultText)
	let parsed = $derived(parser(source))
</script>

<div class="w-screen h-screen grid grid-cols-2 grid-rows-2">
	<div class="w-full h-full col-start-1 row-start-1 col-span-1 row-span-1">
		<textarea name="source" id="source" bind:value={source} class="w-full h-full p-3"></textarea>
	</div>
	<div
		class="w-full h-full col-start-1 row-start-2 col-span-1 row-span-1 leading-snug overflow-scroll bg-stone-900 text-white p-3"
	>
		<pre><code class="text-sm">{JSON.stringify(parsed, null, 2)}</code></pre>
	</div>
	<div class="w-full h-full col-start-2 row-start-1 col-span-1 row-span-2 p-3 overflow-scroll">
		<Markdown {source} />
	</div>
</div>
