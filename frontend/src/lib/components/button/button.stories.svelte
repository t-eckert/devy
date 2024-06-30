<script context="module">
	import Button from "./button.svelte"
	export const meta = {
		title: "components/Button",
		component: Button
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Button {...args}>Button</Button>
</Template>

<Story name="Default"></Story>

<Story
	name="Primary Neutral"
	args={{
		role: "primary",
		behavior: "neutral"
	}}
></Story>

<Story
	name="Primary Positive"
	args={{
		role: "primary",
		behavior: "positive"
	}}
></Story>
