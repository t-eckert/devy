<script context="module">
	import Alert from "./alert.svelte"
	export const meta = {
		title: "components/Panel",
		component: Alert
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Alert {...args} />
</Template>

<Story name="Panel Alert"></Story>
