<script context="module">
	import Menu from "./menu.svelte"
	import Items from "./items.svelte"
	import Link from "./link.svelte"

	export const meta = {
		title: "Menu",
		component: Menu
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
	import Sep from "./sep.svelte"
</script>

<Template let:args>
	<Menu {...args}>
		<Items>
			<Link href="https://www.google.com">Google</Link>
			<Link href="https://www.facebook.com">Facebook</Link>
			<Link href="https://www.twitter.com">Twitter</Link>
			<Link href="https://www.instagram.com">Instagram</Link>
			<Sep />
			<Link href="https://www.linkedin.com">LinkedIn</Link>
		</Items>
	</Menu>
</Template>

<Story name="Example"></Story>
