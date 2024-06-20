<script lang="ts">
	export let authorName: string
	export let authorSlug: string
</script>

<a
	href={`/profiles/${authorSlug}`}
	class="text-xs text-stone-700 hover:text-stone-900 dark:text-stone-400 transition-all"
	>{authorName}</a
>
