<script lang="ts">
	import MenuItem from "./menu.item.svelte"

	let { children, action, name } = $props()
</script>

<MenuItem>
	<form method="POST" {action} class="w-full">
		<button type="submit" class="w-full flex items-start" {name}><slot /></button>
	</form>
</MenuItem>
