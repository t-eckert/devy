<script context="module">
	import Menu from "./menu.svelte"

	export const meta = {
		title: "Menu",
		component: Menu
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Menu {...args} />
</Template>

<Story name="Menu" args={{}} />
