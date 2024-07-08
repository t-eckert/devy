<script lang="ts">
	export let action: string
	export let name: string
</script>

<div
	class="text-sm mx-1 px-2 py-0.5 group flex flex-row rounded items-center pointer-cursor justify-between hover:bg-stone-100"
>
	<form method="POST" {action} class="w-full">
		<button type="submit" class="w-full flex items-start" {name}><slot /></button>
	</form>
</div>
