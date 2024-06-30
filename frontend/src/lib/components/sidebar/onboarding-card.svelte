<script lang="ts">
	import Panel from "$lib/components/panel/panel.svelte"
	import PanelFooter from "$lib/components/panel/panel-footer.svelte"
	import PanelHeader from "$lib/components/panel/panel-header.svelte"
	import { ArrowRight } from "radix-icons-svelte"
	import { PUBLIC_API } from "$env/static/public"
	import sessionStore from "$lib/state/session-store"

	let isLoggedIn = $sessionStore.loggedIn
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
		{#if isLoggedIn}
			<a href="/new/blog" class="flex flex-row gap-1 items-center group"
				><span class="font-medium">Create your blog</span><ArrowRight
					class="group-hover:translate-x-1 transition-all"
				/>
			</a>
		{:else}
			<a href={`${PUBLIC_API}/auth/login`} class="flex flex-row gap-1 items-center group"
				><span class="font-medium">Sign in</span><ArrowRight
					class="group-hover:translate-x-1 transition-all"
				/></a
			>
		{/if}
	</PanelFooter>
</Panel>
