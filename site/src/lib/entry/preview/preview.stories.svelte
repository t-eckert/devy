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

<Story
	name="Example"
	args={{
		post: {
			id: "4c9b32bb-5488-46c3-8cdf-87ca2617b945",
			slug: "hello-world",

			blogSlug: "my-blog",
			blogName: "My Blog",
			authorUsername: "t-eckert",
			authorName: "Tobias Eckert",

			title: "Hello, world!",
			content: "This is my first post!",
			tags: ["hello", "world"],
			createdAt: "2021-08-01T12:00:00Z",
			updatedAt: "2021-08-01T12:00:00Z",
			likes: 20
		}
	}}
></Story>
