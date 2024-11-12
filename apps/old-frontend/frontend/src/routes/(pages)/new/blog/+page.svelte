<script lang="ts">
	import Card from "$lib/components/card.svelte"
	import Main from "$lib/layouts/main.svelte"
	import Input from "$lib/components/input.svelte"
	import Button from "$lib/components/button.svelte"
	import Json from "$lib/utils/json.svelte"

	const { data } = $props()
</script>

<svelte:head>
	<title>Create your blog</title>
</svelte:head>

<Main>
	<div class="mx-auto w-full max-w-3xl mt-12">
		<form class="flex flex-col gap-4" method="post">
			<div>
				<h1 class="font-semibold text-xl text-stone-800 mb-1">Create your blog</h1>
				<p class="text-sm text-stone-600 max-w-sm">
					Devy listens for updates to your blog and publishes markdown files in your repo as blog
					posts.
				</p>
			</div>

			<Card>
				<div class="flex flex-row gap-6">
					<div class="w-1/2">
						<h2 class="text-lg font-semibold text-stone-950">Repo</h2>
						<fieldset aria-label="Public GitHub repositories">
							<legend class="mb-3 text-sm text-stone-600"
								>Select one of your public GitHub Repositories</legend
							>
							<ul class="border rounded-md flex flex-col gap-1 py-1 h-20 sm:h-96 overflow-y-scroll">
								{#each data.repos as repo}
									<li class="grid grid-cols-1">
										<input
											type="radio"
											id={repo.name}
											name="Repo"
											value={repo.clone_url}
											class="peer sr-only"
										/>
										<label
											for={repo.name}
											class="cursor-pointer mx-1 hover:bg-stone-100 px-2 py-0.5 rounded border-2 border-transparent peer-checked:border-indigo-600"
										>
											<div class="flex flex-row gap-2 items-baseline">
												<span class="font-medium">{repo.name}</span>
												<span class="font-mono text-sm text-stone-600">{repo.language}</span>
											</div>
										</label>
									</li>
								{/each}
							</ul>
						</fieldset>
					</div>

					<div class="w-1/2 flex flex-col justify-between">
						<div>
							<div>
								<h2 class="text-lg font-semibold text-stone-950">Name</h2>
								<p class="mb-3 text-sm text-stone-600">
									Choose a name for your blog and a slug to access it.
								</p>
							</div>
							<div class="flex flex-col gap-6">
								<Input name="Name" placeholder="Numerous Thoughts" />
								<Input name="Slug" placeholder="numerous-thoughts" prefix="https://devy.page/" />
							</div>
						</div>

						<div class="p-3 bg-amber-100 rounded">
							<h2 class="font-medium text-amber-950 mb-1">Head's up!</h2>
							<p>
								For Devy to receive webhooks to update your blog, you'll need to <a
									href="https://github.com/apps/devy-github"
									class="underline text-amber-900 hover:text-amber-800"
									>install the app and give it access to your GitHub repositories</a
								>.
							</p>
						</div>
					</div>
				</div>
			</Card>

			<div class="flex justify-end items-center gap-2">
				<Button behavior="positive" role="primary">
					<span class="text-sm font-medium">Submit</span>
				</Button>
				<a
					href="/"
					class="flex flex-row group items-center gap-1 font-medium rounded px-2 py-1 text-sm"
					>Cancel</a
				>
			</div>
		</form>
	</div>
</Main>
