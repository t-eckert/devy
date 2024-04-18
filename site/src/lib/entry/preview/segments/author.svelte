<script lang="ts">
	export let authorName: string
	export let authorSlug: string
</script>

<a href={`/profiles/${authorSlug}`} class="text-xs text-zinc-700 hover:text-zinc-900 transition-all"
	>{authorName}</a
>
