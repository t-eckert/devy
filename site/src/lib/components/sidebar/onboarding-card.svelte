<script lang="ts">
	import Panel from "$lib/components/panel/panel.svelte"
	import PanelFooter from "$lib/components/panel/panel-footer.svelte"
	import PanelHeader from "$lib/components/panel/panel-header.svelte"
	import OnboardingLink from "./onboarding-link.svelte"
	import onboarding, { type OnboardingStore } from "$lib/state/onboarding-store"

	let onboardingState: OnboardingStore

	onboarding.subscribe((value) => (onboardingState = value))
</script>

<Panel>
	<PanelHeader slot="header">
		<h2 class="font-semibold">Welcome to Devy!</h2>
	</PanelHeader>

	<div>
		<p class="mb-2">Devy is a blogging platform powered by GitHub and Markdown.</p>
		<p>
			Create a blog from a repo of markdown files and <code
				class="bg-stone-200 px-0.5 dark:bg-stone-700">git push</code
			> to publish.
		</p>
	</div>

	<PanelFooter slot="footer">
		<OnboardingLink {onboardingState} />
	</PanelFooter>
</Panel>
