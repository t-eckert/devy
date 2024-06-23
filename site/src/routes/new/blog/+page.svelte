<script lang="ts">
	import type { PageData } from "./$types"

	import Main from "$lib/layouts/main.svelte"
	import FormNameInput from "./form-name-input.svelte"
	import FormRepoSelect from "./form-repo-select.svelte"
	import FormSubmit from "./form-submit.svelte"

	import formState, { type FormState, hydrate } from "./form-state"
	import Json from "$lib/utils/json.svelte"
	import { onMount } from "svelte"

	let form: FormState

	onMount(async () => {
		await hydrate("")
	})

	formState.subscribe((value) => {
		form = value
	})
</script>

<Main>
	<div class="mx-auto w-full max-w-2xl">
		<div class="mb-8">
			<h1 class="font-semibold text-xl text-stone-800 mb-1">Create your blog</h1>
			<p class="text-sm max-w-80">
				Get started with your new Devy blog by selecting a repository and naming your blog. You can
				always change these settings later.
			</p>
		</div>

		<form class="flex flex-col gap-8">
			<FormNameInput />
			<FormRepoSelect />
			<FormSubmit />
		</form>
	</div>
</Main>
