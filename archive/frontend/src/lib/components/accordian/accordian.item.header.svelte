<script lang="ts">
	import ChevronRight from "$lib/icons/chevron-right.svelte"
</script>

<div class="mx-3 my-1 flex flex-row">
	<div class="mt-1 mr-1 group-open:rotate-90 transition">
		<ChevronRight />
	</div>
	<slot />
</div>
