<script lang="ts">
	import PlusIcon from "$lib/icons/plus.svelte"
</script>

<div>
	<span
		class="text-xs bg-yellow-100 text-yellow-600 border border-yellow-500 font-medium px-1 py-0.5 rounded"
		>DRAFT</span
	>
</div>
<h1 class="text-2xl font-semibold mt-1 mb-3">Title of the RFC</h1>
<div class="mb-3">
	<div class="text-xs font-medium">Author</div>
	<div class="flex flex-row gap-1 justify-start items-center">
		<span class="text-sm font-medium text-900 bg-stone-100 px-1 py-0.5 rounded">Thomas Eckert</span>
		<button
			class="p-0.5 rounded flex items-center justify-center bg-stone-100 border border-stone-200"
			><PlusIcon /></button
		>
	</div>
</div>
<div class="bg-white pt-8 rounded border border-stone-100">
	<div class="mx-auto w-full max-w-3xl">
		<textarea
			name="editor"
			id="editor"
			placeholder="RFC text"
			class="bg-transparent w-full h-[80vh] resize-none select-none outline-none"
		>
		</textarea>
	</div>
</div>
