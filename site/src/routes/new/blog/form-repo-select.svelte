<script lang="ts">
	import type { Repo } from "$lib/types"
	import formState from "./form-state"

	let limit = true
	let limitLength = 9

	let repos: Repo[] = []
	let repoLoadState: string
	formState.subscribe((value) => {
		repos = value.repos.slice(0, limitLength * (limit ? 1 : -1))
		repoLoadState = value.repoLoadState
	})
</script>

<span>{limit}</span>
<div class="flex flex-col gap-1 items-center">
	<fieldset>
		<legend class="ml-1 mb-2 text-sm font-medium text-gray-900"
			>Select one of your public repositories</legend
		>
		<div class="rounded-md -space-y-px">
			{#if repoLoadState === "loading" || repoLoadState === "idle"}
				{#each Array(limitLength + 1) as _}
					<div
						class="h-8 border border-stone-200 font-medium px-3 py-1 flex flex-row justify-between first:rounded-t-md last:rounded-b-md cursor-none bg-stone-50 animate-pulse"
					></div>
				{/each}
			{:else if repoLoadState === "error"}
				<div
					class="h-80 border border-red-200 font-medium px-3 py-1 flex flex-row justify-center items-center first:rounded-t-md last:rounded-b-md cursor-none bg-red-50"
				>
					<div class="text-red-500">Failed to load repositories</div>
				</div>
			{:else if repoLoadState === "loaded"}
				{#each repos as repo}
					<label
						for={repo.name}
						class="border font-medium px-3 py-1 flex flex-row justify-between first:rounded-t-md last:rounded-b-md cursor-pointer hover:bg-gray-100 transition-colors duration-200 ease-in-out"
					>
						<input type="radio" name="repo" class="sr-only" id={repo.name} value={repo.name} />
						<span>
							{repo.name}
						</span>
					</label>
				{/each}
				<button
					on:click={(e) => {
						e.preventDefault()
						limit = !limit
					}}
					disabled={repoLoadState !== "loaded"}
					class="w-full border font-medium px-3 py-1 flex flex-row items-center justify-center first:rounded-t-md last:rounded-b-md cursor-pointer hover:bg-gray-100 transition-colors duration-200 ease-in-out"
					><div class="font-medium text-sm it">Show all</div></button
				>
			{/if}
		</div>
	</fieldset>
</div>
