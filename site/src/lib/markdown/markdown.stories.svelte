<script context="module">
	import Markdown from "./markdown.svelte"

	export const meta = {
		title: "components/Markdown",
		component: Markdown
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"

	let articleSource = `# Markdown

This is some markdown.

Here is a list:

- Item 1
- Item 2
- Item 3

## Subheading`
</script>

<Template let:args>
	<Markdown {...args} />
</Template>

<Story name="Article" args={{ source: articleSource }}></Story>
