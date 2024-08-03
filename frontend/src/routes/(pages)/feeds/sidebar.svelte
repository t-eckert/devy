<script lang="ts">
	import ArrowRight from "$lib/icons/arrow-right.svelte"
</script>

<section>
	<h1 class="font-semibold mb-1">Welcome to Devy!</h1>
	<p class="mb-2">
		Devy is a blogging platform based on Markdown and Git. Connect a GitHub repository and every <code
			>push</code
		> updates your blog.
	</p>
	<a href="/new/blog" class="flex flex-row gap-1 items-center group">
		<span class="font-medium">Create your blog</span>
		<div class="group-hover:translate-x-1 transition-all">
			<ArrowRight />
		</div>
	</a>
</section>
