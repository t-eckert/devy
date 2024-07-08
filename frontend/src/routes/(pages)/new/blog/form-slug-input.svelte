<script lang="ts">
	import Input from "$lib/components/input.svelte"
	import formState from "./form-state"

	let slug = ""

	formState.subscribe((value) => {
		slug = value.blogSlug
	})
</script>

<Input name="Slug" placeholder="numerous-thoughts" prefix="https://devy.page/" value={slug} />
