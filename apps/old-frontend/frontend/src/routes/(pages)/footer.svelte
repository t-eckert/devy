<script lang="ts">
	import { dev } from "$app/environment"
	import { VERSION } from "$lib"

	import Home from "$lib/components/home.svelte"
	import Main from "$lib/layouts/main.svelte"
	import P from "$lib/elements/p.svelte"
</script>

<footer class="bg-stone-100 dark:bg-stone-950 mt-12">
	<Main>
		<div class="pt-12 pb-3 max-w-sm">
			<div class="mb-3">
				<Home />
			</div>
			<P>A blogging platform where you can write in Markdown and push to GitHub to publish.</P>
		</div>
		<div class="h-full pb-3 gap-3 flex flex-col items-center">
			<div class="flex-1 grid sm:grid-cols-3 w-full">
				<div class="flex flex-col gap-2">
					<a href="/about" class="text-sm font-medium text-stone-600 hover:text-stone-950"
						>About Devy</a
					>
					<a href="/changelog" class="text-sm font-medium text-stone-600 hover:text-stone-950"
						>Changelog</a
					>
					<a
						href="https://polar.sh/t-eckert/devy"
						class="text-sm font-medium text-stone-600 hover:text-stone-950">Support on Polar.sh</a
					>
					<a
						href="https://thomaseckert.dev"
						class="text-sm font-medium text-stone-600 hover:text-stone-950">Developer's Website</a
					>
					{#if dev}
						<a href="/__test/" class="text-sm font-medium text-stone-600 hover:text-stone-950"
							>Tests</a
						>
					{/if}
				</div>
			</div>
			<div>
				<code class="text-slate-700 text-sm">{VERSION}</code>
			</div>
		</div>
	</Main>
</footer>
