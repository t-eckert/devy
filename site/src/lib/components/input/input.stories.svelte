<script context="module">
	import Input from "./input.svelte"
	export const meta = {
		title: "components/Input",
		component: Input
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Input {...args} />
</Template>

<Story
	name="Name Input"
	args={{
		name: "Name",
		placeholder: "Thomas Eckert"
	}}
></Story>

<Story
	name="Name Input with Description"
	args={{
		name: "Name",
		placeholder: "Thomas Eckert",
		description: "Please enter your full name."
	}}
></Story>

<Story
	name="Name with error message"
	args={{
		name: "Name",
		placeholder: "Thomas Eckert",
		description: "Please enter your full name.",
		errorMessage: "That is not your name!"
	}}
></Story>

<Story
	name="Optional Name Input"
	args={{
		name: "Name",
		optional: true,
		placeholder: "Thomas Eckert"
	}}
></Story>
