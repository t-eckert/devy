<script context="module">
	import Logo from "./logo.svelte"

	export const meta = {
		title: "components/Logo",
		component: Logo
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Logo {...args} />
</Template>

<Story name="Basic Logo" args={{ height: 100 }}></Story>
