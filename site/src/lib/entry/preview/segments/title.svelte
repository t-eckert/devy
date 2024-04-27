<script lang="ts">
	export let title: string
	export let blogSlug: string
	export let postSlug: string
</script>

<a
	href={`/${blogSlug}/${postSlug}`}
	class="font-medium text-stone-700 hover:text-stone-950 transition-all">{title}</a
>
