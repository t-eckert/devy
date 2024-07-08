<script lang="ts">
	import Card from "$lib/components/card.svelte"
	import Main from "$lib/layouts/main.svelte"
	import FormNameInput from "./form-name-input.svelte"
	import FormRepoSelect from "./form-repo-select.svelte"
	import FormSlugInput from "./form-slug-input.svelte"
	import FormSubmit from "./form-submit.svelte"
</script>

<Main>
	<div class="mx-auto w-full max-w-3xl">
		<form class="flex flex-col gap-4">
			<h1 class="font-semibold text-xl text-stone-800">Create your blog</h1>

			<Card>
				<div class="flex flex-row justify-between">
					<FormRepoSelect />
					<div class="flex flex-col gap-6">
						<FormNameInput />
						<FormSlugInput />
					</div>
				</div>
			</Card>

			<FormSubmit />
		</form>
	</div>
</Main>
