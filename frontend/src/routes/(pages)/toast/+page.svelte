<script lang="ts">
	import Button from "$lib/components/button.svelte"
	import toast from "svelte-french-toast"

	const onclick = () => {
		toast.success("New toast!", { position: "bottom-right" })
	}
</script>

<Button {onclick}>Make toast!</Button>
