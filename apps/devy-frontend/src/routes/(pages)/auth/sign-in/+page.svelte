<script lang="ts">
	import Centered from "$lib/layouts/centered.svelte"
	import Panel from "$lib/components/panel/panel.svelte"
	import PanelBar from "$lib/components/panel/panel-bar.svelte"
	import PanelBody from "$lib/components/panel/panel-body.svelte"
	import PanelActions from "$lib/components/panel/panel-actions.svelte"
	import Button from "$lib/components/button.svelte"
	import GithubLogo from "$lib/icons/github-logo.svelte"

	let { data } = $props()
	let { providers } = data
</script>

<Centered>
	<Panel>
		{#snippet bar()}<PanelBar title="Sign in" />{/snippet}
		{#snippet body()}
			<PanelBody>
				<div class="my-3 flex min-w-64 flex-col items-center gap-3">
					<Button href={providers.GitHub}>
						<span class="flex flex-row items-center gap-2">
							<GithubLogo />
							<span class="text-sm font-medium">Sign in with GitHub Auth</span>
						</span>
					</Button>

					<div class="-mb-1 mt-1 flex flex-row items-center gap-3">
						<span class="h-0 w-8 border-b-2 border-b-stone-300"></span>
						<span class="italic text-stone-500">or</span>
						<span class="h-0 w-8 border-b-2 border-b-stone-300"></span>
					</div>

					<div class="-mb-1">
						<Button role="tertiary" href="/auth/sign-up">
							<span class="flex flex-row items-center gap-2">
								<span class="text-sm font-medium">Sign up</span>
							</span>
						</Button>
					</div>
				</div>
			</PanelBody>
		{/snippet}
		{#snippet actions()}
			<PanelActions>
				<Button href="/" role="tertiary"
					><span class="text-sm font-medium">Return home</span></Button
				>
			</PanelActions>
		{/snippet}
	</Panel>
</Centered>
