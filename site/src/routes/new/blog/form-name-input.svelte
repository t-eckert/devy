<script lang="ts">
	import Card from "$lib/components/card/card.svelte"
	import Input from "$lib/components/input/input.svelte"
	import formState, { setBlogName } from "./form-state"

	let name = ""
	let slug = ""

	formState.subscribe((value) => {
		name = value.blogName
		slug = value.blogSlug
	})
</script>

<div>
	<h2 class="font-semibold text-stone-800 mb-3">Name your blog</h2>
	<Card>
		<div class="flex flex-row gap-3">
			<Input name="Name" placeholder="Numerous Thoughts" value={name} onUpdate={setBlogName} />
			<Input name="Slug" placeholder="numerous-thoughts" prefix="https://devy.page/" value={slug} />
		</div>
	</Card>
</div>
