<script lang="ts">
	import Card from "$lib/components/card/card.svelte"
	import { RadioGroup } from "bits-ui"
	import type { Repo } from "$lib/types"
	import formState from "./form-state"

	let repos: Repo[] = []
	formState.subscribe((value) => {
		repos = value.repos
	})
</script>

<div class="flex flex-col">
	<h2 class="font-medium">Select a repository</h2>
	<p class="text-sm mb-4">
		Lorem ipsum dolor sit amet consectetur adipisicing elit. Odit itaque tenetur harum optio!
		Perferendis fuga quisquam, rem eaque perspiciatis enim rerum deserunt in dignissimos error. Quod
		eos odio dolore ea!
	</p>
	<Card>
		<div class="flex flex-col gap-3">
			<RadioGroup.Root class="grid grid-cols-3 gap-4">
				{#each repos.slice(0, 9) as repo}
					<button class="flex flex-col rounded-md border border-zinc-100 p-2">
						<span class="font-medium text-zinc-700">{repo.name}</span>
					</button>
				{/each}
			</RadioGroup.Root>
			<button
				class="w-full rounded-b-md flex items-center justify-center py-2 text-sm font-medium border-t border-t-200"
				>Load more</button
			>
		</div>
	</Card>
</div>
