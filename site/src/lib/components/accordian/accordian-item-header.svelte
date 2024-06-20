<script lang="ts">
	import { ChevronRight } from "radix-icons-svelte"
</script>

<div class="mx-3 my-1 flex flex-row">
	<ChevronRight class="mt-1 mr-1 group-open:rotate-90 transition" /><slot />
</div>
