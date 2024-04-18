<script lang="ts">
	export let title: string
	export let blogSlug: string
	export let postSlug: string
</script>

<a
	href={`${blogSlug}/${postSlug}`}
	class="font-medium text-zinc-700 hover:text-zinc-950 transition-all">{title}</a
>
