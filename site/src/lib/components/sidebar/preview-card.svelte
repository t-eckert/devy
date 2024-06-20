<script lang="ts">
	import Panel from "$lib/components/panel/panel.svelte"
	import PanelHeader from "$lib/components/panel/panel-header.svelte"
</script>

<Panel>
	<PanelHeader slot="header">
		<strong class="text-amber-600">Devy is still in preview</strong>
	</PanelHeader>
	<div class="flex flex-col gap-3">
		<p>Thank you for checking it out while it's still in its early stages.</p>
		<p>
			Sharing your <a href="/feedback" class="underline font-medium">feedback</a> is an amazing way you
			can help make this site better.
		</p>
	</div>
</Panel>
