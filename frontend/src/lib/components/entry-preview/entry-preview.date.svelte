<script lang="ts">
	import RelativeDate from "$lib/utils/relative-date.svelte"
	export let date: string
</script>

<div class="text-xs dark:text-stone-300"><RelativeDate {date} /></div>
