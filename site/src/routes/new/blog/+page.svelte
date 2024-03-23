<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layout/main.svelte"
	import Button from "$lib/element/button.svelte"

	export let data: PageData
</script>

<Main>
	<form
		method="POST"
		class="flex flex-col gap-4 w-full mx-auto bg-white p-3 items-start max-w-2xl rounded-md shadow"
	>
		<h1 class="font-semibold">Create a new blog</h1>
		<div class="flex flex-col">
			<label for="blog-name" class="font-medium">Name</label>
			<p class="text-sm">Choose a name for your blog.</p>
			<input type="text" name="blog-name" placeholder="My Blog" required />
		</div>
		<div class="flex flex-col">
			<label for="githubRepo" class="font-medium">GitHub Repo</label>
			<p class="text-sm">Select a repository to push posts to.</p>
			<select name="githubRepo">
				{#each data.props.githubRepos as { name, full_name }}
					<option value={full_name}>{name}</option>
				{/each}
			</select>
		</div>
		<Button>Create Blog</Button>
	</form>
</Main>
