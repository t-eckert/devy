<script context="module">
	import ProfilePreview from "./profile-preview.svelte"
	export const meta = {
		title: "components/Profile Preview",
		component: ProfilePreview
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<ProfilePreview {...args} />
</Template>

<Story
	name="With Photo and bio"
	args={{
		profile: {
			displayName: "Bethany Styles",
			avatarUrl:
				"https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=2487&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			createdAt: "2021-01-01",
			bio: "I haven't met a container I couldn't orchestrate."
		},
		blogs: [
			{ name: "First Blog", slug: "first-blog" },
			{ name: "Second Blog", slug: "second-blog" }
		],
		user: { username: "johndoe" }
	}}
></Story>
