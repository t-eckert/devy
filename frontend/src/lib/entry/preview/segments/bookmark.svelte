<script lang="ts">
	import entryStore from "../entry-store"
	import { Bookmark } from "radix-icons-svelte"

	entryStore.subscribe(({ entry }) => {})
</script>

<button class="rounded group">
	<Bookmark
		class="w-4 aspect-square text-stone-500 group-hover:-translate-y-1 group-hover:text-stone-950 transition-all"
	/>
</button>
