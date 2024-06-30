<script context="module">
	import Menu from "./menu.svelte"
	import Link from "./link.svelte"

	export const meta = {
		title: "components/Menu",
		component: Menu
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
	import Sep from "./sep.svelte"
</script>

<Template let:args>
	<Menu {...args}>
		<Link href="https://www.google.com">Google</Link>
		<Link href="https://www.facebook.com">Facebook</Link>
		<Link href="https://www.twitter.com">Twitter</Link>
		<Link href="https://www.instagram.com">Instagram</Link>
		<Sep />
		<Link href="https://www.linkedin.com">LinkedIn</Link>
	</Menu>
</Template>

<Story name="Example"></Story>
<Story name="Top-Start" args={{ side: "top", align: "start" }}></Story>
