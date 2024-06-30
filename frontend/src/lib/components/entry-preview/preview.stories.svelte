<script context="module">
	import Preview from "./story.svelte"

	export const meta = {
		title: "Components/Entry Preview",
		component: Preview
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Preview {...args} />
</Template>

<Story
	name="Preview Variants"
	args={{
		entry: {
			postSlug: "hello-world",

			blogSlug: "my-blog",
			blogName: "My Blog",
			authorUsername: "t-eckert",
			authorName: "Tobias Eckert",

			title: "Hello, world!",
			content: "This is my first post!",
			tags: ["hello", "world"],
			createdAt: "2021-08-01T12:00:00Z",
			updatedAt: "2021-08-01T12:00:00Z",
			likes: 20000,

			liked: true
		}
	}}
></Story>
