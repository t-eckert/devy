<script context="module">
	import Card from "./card.svelte"
	export const meta = {
		title: "components/Card",
		component: Card
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template>
	<Card>
		<p class="w-96 text-stone-900">
			Lorem ipsum dolor sit amet consectetur adipisicing elit. Inventore eos magnam doloribus! Iste
			unde nobis consequuntur reprehenderit provident facere, soluta at illum, quaerat nostrum modi?
			Omnis voluptas laudantium sed libero.
		</p>
	</Card>
</Template>

<Story name="Text"></Story>
