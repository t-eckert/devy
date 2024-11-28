<script lang="ts">
	import GitHubLogo from "$lib/icons/github-logo.svelte"
	import ArrowRight from "$lib/icons/arrow-right.svelte"
	import Code from "$lib/elements/code.svelte"
	import Button from "$lib/components/button.svelte"

	import { getUser } from "$lib/state/user.svelte"

	let user = getUser()
</script>

<div class="flex flex-col items-start gap-6">
	<section class="dark:text-zinc-100 flex flex-col items-start">
		<h1 class="font-semibold mb-1">Welcome to Devy!</h1>
		<p class="mb-1 text-sm">Devy is a blogging platform based on Markdown and Git.</p>
		<p class="mb-2 text-sm">
			Connect a GitHub repository, add Markdown files, and every <Code>push</Code> updates your blog.
		</p>

		{#if user.isAuthenticated}
			<Button href="/new/blog" role="primary">
				<div class="flex flex-row gap-1 items-center group">
					<span class="font-medium text-sm">Deploy your blog</span>
					<div class="group-hover:translate-x-1 transition-all">
						<ArrowRight />
					</div>
				</div>
			</Button>
		{:else}
			<Button href="/auth/sign-in" role="primary">
				<div class="flex flex-row gap-1 items-center group">
					<span>
						<GitHubLogo />
					</span>
					<span class="font-medium text-sm">Sign up with GitHub</span>
				</div>
			</Button>
		{/if}
	</section>
</div>
