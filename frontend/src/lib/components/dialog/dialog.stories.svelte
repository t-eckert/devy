<script context="module">
	import Dialog from "./dialog.svelte"
	import Panel from "$lib/components/panel/panel.svelte"
	export const meta = {
		title: "components/Dialog",
		component: Dialog
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template let:args>
	<Dialog {...args}>
		<Panel>
			<p class="w-96 text-stone-900">
				Lorem ipsum dolor sit amet consectetur adipisicing elit. Inventore eos magnam doloribus!
				Iste unde nobis consequuntur reprehenderit provident facere, soluta at illum, quaerat
				nostrum modi? Omnis voluptas laudantium sed libero.
			</p>
		</Panel>
	</Dialog>
</Template>

<Story name="Text"></Story>
