<script context="module">
	import Avatar from "./avatar.svelte"
	export const meta = {
		title: "components/Avatar",
		component: Avatar
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Avatar {...args} />
</Template>

<Story
	name="With image"
	args={{
		src: "https://images.unsplash.com/photo-1513152697235-fe74c283646a",
		display_name: "Tobias Eckert"
	}}
></Story>

<Story
	name="Without image"
	args={{
		display_name: "Tobias Eckert"
	}}
></Story>
