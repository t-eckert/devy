<script lang="ts">
	import ArrowRight from "$lib/icons/arrow-right.svelte"
</script>

<div class="flex flex-col items-start gap-6">
	<section>
		<h1 class="font-semibold mb-1">Welcome to Devy!</h1>
		<p class="mb-1 text-sm">Devy is a blogging platform based on Markdown and Git.</p>
		<p class="mb-2 text-sm">
			Connect a GitHub repository, add Markdown files, and every <code class="bg-stone-200"
				>push</code
			> updates your blog.
		</p>
		<a href="/new/blog" class="flex flex-row gap-1 items-center group">
			<span class="font-medium text-sm">Deploy your blog</span>
			<div class="group-hover:translate-x-1 transition-all">
				<ArrowRight />
			</div>
		</a>
	</section>

	<section class="p-3 bg-amber-100 rounded-md shadow-md border border-amber-200">
		<p class="text-sm">
			Devy is still in the early stages of development. If you encounter a problem, please
			<a
				href="https://github.com/t-eckert/devy/issues/new/choose"
				class="text-sm font-medium text-stone-600 hover:text-stone-950 underline">open an issue</a
			> on the GitHub repo.
		</p>
	</section>
</div>
