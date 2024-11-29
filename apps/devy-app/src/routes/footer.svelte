<script lang="ts">
	import Logo from "$lib/components/logo.svelte";
	import Bounded from "$lib/layouts/bounded.svelte";

	let props = $props();
</script>

{#snippet item(href: string, text: string)}
	<li>
		<a {href} class="text-sm font-medium text-stone-600 hover:text-stone-950">{text}</a>
	</li>
{/snippet}

<footer class="h-full min-h-[90vh] border-t border-t-stone-200 bg-stone-100">
	<Bounded>
		<div class="mt-24 flex h-full flex-col items-center gap-4">
			<section class="grid w-full flex-1 grid-cols-4 gap-6">
				<div>
					<div class="mb-2"><Logo height={42} /></div>
					<p class="text-sm">
						Devy lets you create a blog from a GitHub repo, push markdown to that repo, and publish
						that markdown as blog posts.
					</p>
				</div>
				<div class="text-sm">
					<h2 class="font-medium">Devy</h2>
					<ul>
						{@render item("/about", "About")}
						{@render item("/changelog", "Changelog")}
					</ul>
				</div>
				<div>
					<ul></ul>
				</div>
				<div>
					<ul></ul>
				</div>
			</section>
			<div class="text-sm text-stone-700"><code>v2024.11.28</code></div>
		</div>
	</Bounded>
</footer>
