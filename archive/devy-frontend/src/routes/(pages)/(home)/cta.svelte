<script lang="ts">
	import ArrowRight from "$lib/icons/arrow-right.svelte"
	import Button from "$lib/components/button.svelte"

	import { getSession } from "$lib/state/session.svelte"

	let session = getSession()
</script>

<div class="flex flex-col items-start gap-6">
	<section class="flex flex-col items-start dark:text-zinc-100">
		<h1 class="mb-1 font-semibold">Welcome to Devy!</h1>
		<p class="mb-1 text-sm">Devy is a blogging platform based on Markdown and Git.</p>
		<p class="mb-2 text-sm">
			Connect a GitHub repository, add Markdown files, and every push updates your blog.
		</p>

		{#if session.isAuthenticated}
			<Button href="/new/blog" role="primary">
				<div class="group flex flex-row items-center gap-1">
					<span class="text-sm font-medium">Deploy your blog</span>
					<div class="transition-all group-hover:translate-x-1">
						<ArrowRight />
					</div>
				</div>
			</Button>
		{:else}
			<Button href="/auth/sign-up" role="primary">
				<div class="group flex flex-row items-center gap-1">
					<span class="text-sm font-medium">Sign up</span>
					<div class="transition-all group-hover:translate-x-1">
						<ArrowRight />
					</div>
				</div>
			</Button>
		{/if}
	</section>
</div>
