<script lang="ts">
	import { dev } from "$app/environment"

	let version = "v2024.07.16"

	import Home from "$lib/components/home.svelte"
	import Main from "$lib/layouts/main.svelte"
	import P from "$lib/elements/p.svelte"
</script>

<footer class="h-96 bg-stone-100 mt-12">
	<Main>
		<div class="h-full pt-12 pb-3 gap-3 flex flex-col items-center">
			<div class="flex-1 grid sm:grid-cols-3 w-full">
				<div>
					<div class="mb-3">
						<Home />
					</div>
					<P
						>An open source blogging platform where you can write in Markdown and push to GitHub to
						publish.</P
					>
					<div class="flex flex-col gap-2">
						<a
							href="https://github.com/t-eckert/devy"
							class="text-sm font-medium text-stone-600 hover:text-stone-950">GitHub</a
						>
						<a
							href="https://github.com/t-eckert/devy/issues/new/choose"
							class="text-sm font-medium text-stone-600 hover:text-stone-950">Open an issue</a
						>
						<a
							href="https://polar.sh/t-eckert/devy"
							class="text-sm font-medium text-stone-600 hover:text-stone-950">Support on Polar.sh</a
						>
						<a
							href="https://discord.gg/qUJ54EEC5g"
							class="text-sm font-medium text-stone-600 hover:text-stone-950">Discord</a
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
				<div></div>
				<div></div>
			</div>
			<div>
				<code class="text-slate-700 text-sm">{version}</code>
			</div>
		</div>
	</Main>
</footer>
