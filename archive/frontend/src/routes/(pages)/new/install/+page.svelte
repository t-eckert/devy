<script lang="ts">
	import Card from "$lib/components/card.svelte"
</script>

<div class="flex flex-col gap-4">
	<div>
		<h1 class="font-medium text-xl">Install</h1>
		<p class="text-sm w-52">First, you need to install Devy to your GitHub account.</p>
	</div>
	<Card>
		<div class="p-6 flex flex-row gap-8">
			<div class="w-72">
				<h2 class="font-medium">Why?</h2>
				<p class="text-sm mb-2">
					Devy automatically syncs markdown from your GitHub repository to your blog. To do this,
					Devy needs to be able to receive Webhooks events when your repository is updated.
				</p>
			</div>
			<div class="w-72">
				<h2 class="font-medium">What information will be shared?</h2>
				<p class="text-sm mb-2">Devy receives the following information from GitHub:</p>
				<ul class="text-sm mb-2 list-outside ml-4 list-disc">
					<li>Read/Write access to Deployments</li>
					<li>
						Read only access to repository contents so that the repo can be cloned for uploading.
					</li>
					<li>Repo Metadata</li>
				</ul>
				<p class="text-sm">
					You are able to limit sharing this data for to own certain repositories.
				</p>
			</div>
		</div>
	</Card>
	<div class="flex justify-between items-center gap-2">
		<a
			href="/"
			class="flex flex-row group items-center gap-1 font-medium rounded px-2 py-1 text-sm"
		>
			<span>Back</span></a
		>

		<a
			href="https://github.com/apps/devy-github"
			class="flex flex-row group items-center gap-1 font-medium rounded text-zinc-700 px-2 py-1 text-sm"
		>
			<span>Install</span>
		</a>
	</div>
</div>
