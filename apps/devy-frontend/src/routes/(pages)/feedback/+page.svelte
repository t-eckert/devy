<script lang="ts">
	import Panel from "$lib/components/panel/panel.svelte"
	import PanelActions from "$lib/components/panel/panel-actions.svelte"
	import PanelBar from "$lib/components/panel/panel-bar.svelte"
	import PanelBody from "$lib/components/panel/panel-body.svelte"
	import Button from "$lib/components/button.svelte"
	import Centered from "$lib/layouts/centered.svelte"
</script>

<Centered>
	<form action="POST">
		<Panel>
			{#snippet bar()}<PanelBar title="Feedback" />{/snippet}
			{#snippet body()}
				<PanelBody>
					<div class="flex flex-col">
						<input type="text" />
						<textarea name="feedback" id=""></textarea>
					</div>
				</PanelBody>
			{/snippet}
			{#snippet actions()}
				<PanelActions>
					<Button tone="positive"><span class="text-sm font-medium">Submit</span></Button>
					<Button role="tertiary" href="/">
						<span class="text-sm font-medium">Return home</span>
					</Button>
				</PanelActions>
			{/snippet}
		</Panel>
	</form>
</Centered>
