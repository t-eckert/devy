<script lang="ts">
	interface Props {
		title: string
	}

	const { title }: Props = $props()
</script>

<meta property="og:title" content={title} />
<meta property="og:image" content="https://devy.page/og-img.png" />
