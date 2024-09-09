<script lang="ts">
	import ArrowRight from "$lib/icons/arrow-right.svelte"
	import Code from "$lib/elements/code.svelte"
</script>

<div class="flex flex-col items-start gap-6">
	<section class="dark:text-zinc-100">
		<h1 class="font-semibold mb-1">Welcome to Devy!</h1>
		<p class="mb-1 text-sm">Devy is a blogging platform based on Markdown and Git.</p>
		<p class="mb-2 text-sm">
			Connect a GitHub repository, add Markdown files, and every <Code>push</Code> updates your blog.
		</p>
		<a href="/new/blog" class="flex flex-row gap-1 items-center group">
			<span class="font-medium text-sm">Deploy your blog</span>
			<div class="group-hover:translate-x-1 transition-all">
				<ArrowRight />
			</div>
		</a>
	</section>
</div>
