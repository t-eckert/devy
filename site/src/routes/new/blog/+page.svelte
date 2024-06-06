<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import Button from "$lib/elements/button.svelte"
	import Card from "$lib/components/card/card.svelte"

	export let data: PageData
</script>

<Main>
	<div class="max-w-2xl mx-auto mb-4">
		<Card>
			<div class="m-3">
				<p>
					<strong class="text-orange-600">Heads-up</strong>, for Devy to work properly, you'll need
					to
					<a href="https://github.com/apps/devy-github" class="underline"
						>install it to your GitHub</a
					>.
				</p>
			</div>
		</Card>
	</div>

	<form
		method="POST"
		class="flex flex-col gap-4 w-full mx-auto bg-white p-3 items-start max-w-2xl rounded-md shadow"
	>
		<h1 class="font-semibold">Create a new blog</h1>
		<div class="flex flex-col">
			<label for="blog-name" class="font-medium">Name</label>
			<p class="text-sm mb-1">Choose a name for your blog.</p>
			<input
				type="text"
				name="blog-name"
				placeholder="My Blog"
				class="border-b px-2 py-1"
				required
			/>
		</div>
		<div class="flex flex-col">
			<label for="github-repo" class="font-medium">GitHub Repo</label>
			<p class="text-sm mb-1">Select one of your public repositories to push posts from.</p>
			<select name="repo-url" class="border rounded-md pl-2 py-1">
				{#each data.props.githubRepos as { name, clone_url: url }}
					<option value={url}>{name}</option>
				{/each}
			</select>
		</div>
		<Button>Create Blog</Button>
	</form>
</Main>
