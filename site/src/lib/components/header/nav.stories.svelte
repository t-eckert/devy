<script context="module">
	import Nav from "./nav.svelte"
	export const meta = {
		title: "Header/Nav",
		component: Nav
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Nav {...args} />
</Template>

<Story name="Navigation Menu"></Story>
