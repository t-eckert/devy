<script lang="ts">
	import { ArrowRight, ArrowTopRight } from "radix-icons-svelte"
	import { type OnboardingStore } from "$lib/state/onboarding-store"
	import { PUBLIC_API } from "$env/static/public"

	export let onboardingState: OnboardingStore
</script>

{#if onboardingState.isLoggedIn && onboardingState.hasDevyInstalled}
	<a href="/new/blog" class="mt-8 flex flex-row gap-1 items-center group"
		><span class="font-medium">Create your blog</span><ArrowRight
			class="group-hover:translate-x-1 transition-all"
		/>
	</a>
{:else if onboardingState.isLoggedIn && !onboardingState.hasDevyInstalled}
	<a href="https://github.com/apps/devy-github" class="mt-8 flex flex-row gap-1 items-center group"
		><span class="font-medium">Add Devy to GitHub</span><ArrowTopRight
			class="group-hover:translate-x-1 group-hover:-translate-y-1 transition-all"
		/>
	</a>
{:else}
	<a href={`${PUBLIC_API}/auth/login`} class="mt-8 flex flex-row gap-1 items-center group"
		><span class="font-medium">Sign in</span><ArrowTopRight
			class="group-hover:translate-x-1 group-hover:-translate-y-1 transition-all"
		/>
	</a>
{/if}
