<script lang="ts">
	import type { PageData } from "./$types"
	import Button from "$lib/components/button.svelte"

	const { data }: { data: PageData } = $props()
</script>

<h1 class="font-semibold text-xl text-stone-800 mb-3">Manage your blogs</h1>
<form method="POST" action="?/deleteBlog">
	<ul class="flex flex-col gap-2">
		{#each data.blogs as blog}
			<li class="flex flex-row justify-between">
				<span>{blog.name}</span>
				<Button role="secondary" behavior="negative" value={blog.slug} name="blog-slug"
					><span class="text-sm font-medium">Delete</span></Button
				>
			</li>
		{/each}
	</ul>
</form>
