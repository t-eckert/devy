<script lang="ts">
	import Panel from "../panel.svelte"
	import PanelHeader from "../panel-header.svelte"
	import PanelFooter from "../panel-footer.svelte"
</script>

<Panel>
	<PanelHeader slot="header" style="alert">
		<h1 class="text-sm font-medium">Unable to delete blog</h1>
	</PanelHeader>
	<div class="w-96">
		<p class="mb-3">
			Something went wrong and we were unable to save your changes. Please try again.
		</p>
		<p>
			We have receive the details of the error and will be reaching out shortly to remedy the
			problem. Thank you for your patience.
		</p>
	</div>
	<PanelFooter slot="footer">
		<div class="flex flex-row justify-end gap-3">
			<button class="px-3 text-sm py-0.5 text-white bg-red-600 rounded font-medium"
				>Try Delete Again</button
			>
			<button class="px-2 text-sm py-0.5 rounded font-medium">Cancel</button>
		</div>
	</PanelFooter>
</Panel>
