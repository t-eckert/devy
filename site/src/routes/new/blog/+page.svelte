<script lang="ts">
	import Main from "$lib/layouts/main.svelte"
	import Card from "$lib/components/card/card.svelte"
	import { ArrowUp, ArrowRight } from "radix-icons-svelte"
	import type { PageData } from "./$types"
	import { RadioGroup } from "bits-ui"
	import Input from "$lib/components/input/input.svelte"
	import Json from "$lib/utils/json.svelte"

	export let data: PageData
	let { repos } = data
</script>

<Main>
	<div class="mb-8">
		<h1 class="font-medium text-xl">Create your blog</h1>
		<p class="text-sm max-w-80">
			Lorem ipsum dolor sit amet consectetur adipisicing elit. Provident, cupiditate soluta atque,
			quidem cum esse laboriosam, eius maxime quis doloribus beatae quaerat repudiandae nihil
			suscipit error quasi tempore aperiam possimus?
		</p>
	</div>

	<form class="flex flex-col gap-8">
		<div class="flex flex-row gap-8">
			<div class="w-full max-w-80">
				<h2 class="font-medium">Select a repository</h2>
				<p class="text-sm">
					Lorem ipsum dolor sit amet consectetur adipisicing elit. Odit itaque tenetur harum optio!
					Perferendis fuga quisquam, rem eaque perspiciatis enim rerum deserunt in dignissimos
					error. Quod eos odio dolore ea!
				</p>
			</div>
			<div class="flex-1">
				<Card>
					<div class="p-3 flex flex-col gap-4">
						<RadioGroup.Root class="grid grid-cols-3 gap-4">
							{#each repos.slice(0, 9) as repo}
								<button class="flex flex-col min-h-36 rounded-md border border-zinc-100 p-2">
									<span class="font-medium text-zinc-700">{repo.name}</span>
									{#if repo.description}
										<span class="text-start text-sm">{repo.description}</span>
									{/if}
									{#if repo.language}
										<span class="text-xs text-zinc-500">{repo.language}</span>
									{/if}
								</button>
							{/each}
						</RadioGroup.Root>
					</div>
					<button
						class="w-full rounded-b-md flex items-center justify-center py-2 text-sm font-medium border-t border-t-200"
						>Load more</button
					>
				</Card>
			</div>
		</div>

		<div class="flex flex-row gap-8">
			<div class="w-full max-w-80">
				<h2 class="font-medium">Name your blog</h2>
				<p class="text-sm">
					Lorem ipsum dolor sit amet consectetur, adipisicing elit. Similique consectetur quis
					magnam reiciendis modi in veritatis incidunt odit! Pariatur nesciunt ab, omnis numquam
					explicabo est fugit non nisi suscipit. Excepturi?
				</p>
			</div>
			<div class="w-80">
				<Card>
					<div class="p-3 flex flex-row items-start gap-4">
						<Input name="Name" placeholder="My blog" />
						<Input name="Slug" placeholder="my-blog" />
					</div>
				</Card>
			</div>
		</div>

		<div class="flex justify-end items-center gap-2">
			<button
				class="flex flex-row group items-center gap-1 font-medium rounded px-2 py-1 text-sm bg-green-600 text-white"
			>
				<span>Submit</span>
				<ArrowUp class="w-4 h-4 group-hover:-translate-y-1 transition" />
			</button>
			<a
				href="/"
				class="flex flex-row group items-center gap-1 font-medium rounded px-2 py-1 text-sm"
				>Cancel</a
			>
		</div>
	</form>
</Main>
