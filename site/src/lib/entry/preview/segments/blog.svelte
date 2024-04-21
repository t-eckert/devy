<script lang="ts">
	export let blogName: string
	export let blogSlug: string
</script>

<a href={`${blogSlug}`} class="text-xs text-stone-500">{blogName}</a>
