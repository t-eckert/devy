<script lang="ts">
	import { fly } from "svelte/transition"
	import Panel from "$lib/components/panel/panel.svelte"
	import PanelHeader from "$lib/components/panel/panel-header.svelte"
	import PanelFooter from "$lib/components/panel/panel-footer.svelte"

	export let status: any
	export let error: any
</script>

<div class="w-full flex items-center justify-center">
	<div transition:fly class="w-full mx-auto">
		<div class="w-full max-w-lg">
			<Panel>
				<PanelHeader style="alert" slot="header">Error</PanelHeader>

				<div class="flex flex-col">
					<div class="text-8xl font-mono">{status}</div>

					<div class="-mt-1">{error.message}</div>
				</div>

				<PanelFooter slot="footer">
					<div class="flex flex-row gap-2 justify-end items-center">
						<a
							class="px-2 py-0.5 bg-stone-800 rounded text-stone-50 font-medium text-sm hover:bg-stone-950 transition-all"
							href="/">Go home</a
						>
						<a class="px-2 py-0.5 rounded text-stone-700 font-medium text-sm" href="/feedback"
							>Give feedback</a
						>
					</div>
				</PanelFooter>
			</Panel>
		</div>
	</div>
</div>
