<script lang="ts">
	interface Props {
		title: string
		description?: string
	}

	const { title, description }: Props = $props()
</script>

<title>{title}</title>
{#if description}
	<meta name="description" content={description} />
{/if}

<meta property="og:type" content="website" />
<meta property="og:title" content={title} />
<meta property="og:image" content="https://devy.page/og-img.png" />

<!-- Twitter Meta Tags -->
<meta name="twitter:card" content="summary_large_image" />
<meta property="twitter:domain" content="devy.page" />
<meta name="twitter:title" content={title} />
<meta name="twitter:description" content={description} />
<meta name="twitter:image" content="https://devy.page/og-img.png" />
