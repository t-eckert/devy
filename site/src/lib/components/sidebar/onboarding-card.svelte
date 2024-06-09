<script lang="ts">
	import Card from "$lib/components/card/card.svelte"
	import OnboardingLink from "./onboarding-link.svelte"
	import onboarding, { type OnboardingStore } from "$lib/state/onboarding-store"

	let onboardingState: OnboardingStore

	onboarding.subscribe((value) => (onboardingState = value))
</script>

<Card styling="p-2 border bg-white border-stone-200 dark:border-stone-700">
	<h2 class="font-medium text-xl mb-1">Welcome to Devy!</h2>
	<p class="mb-2">Devy is a blogging platform powered by GitHub and Markdown.</p>
	<p>
		Create a blog from a repo of markdown files and <code
			class="bg-stone-200 px-0.5 dark:bg-stone-700">git push</code
		> to publish.
	</p>
	<OnboardingLink {onboardingState} />
</Card>
