<script context="module">
	import Preview from "./preview.svelte"

	export const meta = {
		title: "Post/Preview",
		component: Preview
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Preview {...args} />
</Template>

<Story name="Example"></Story>
