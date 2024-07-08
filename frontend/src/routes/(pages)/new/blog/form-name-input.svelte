<script lang="ts">
	import Input from "$lib/components/input.svelte"
	import formState, { setBlogName } from "./form-state"

	let name = ""
	formState.subscribe((value) => {
		name = value.blogName
	})
</script>

<Input name="Name" placeholder="Numerous Thoughts" value={name} onUpdate={setBlogName} />
