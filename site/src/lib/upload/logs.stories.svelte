<script context="module">
	import Logs from "./logs.svelte"

	export const meta = {
		title: "Upload/Logs",
		component: Logs
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Logs {...args} />
</Template>

<Story
	name="Upload in progress"
	args={{
		logs: [
			"INFO: Upload received by uploader.",
			"INFO: Repository cloned.",
			"INFO: Uploaded title",
			"INFO: Cleaning up repository.",
			"INFO: Upload complete."
		]
	}}
></Story>
