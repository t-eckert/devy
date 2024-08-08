<script lang="ts">
	interface Props {
		title: string
		blogSlug: string
		postSlug: string
	}

	const { title, blogSlug, postSlug }: Props = $props()
</script>

<a
	href={`/${blogSlug}/${postSlug}`}
	class="font-medium text-stone-700 hover:text-stone-950 dark:text-stone-200 transition-all"
	>{title}</a
>
