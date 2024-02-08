<script lang="ts">
	import { DropdownMenu } from "bits-ui"
</script>

<DropdownMenu.Item
	class="text-sm mx-1 px-2 py-0.5 group flex flex-row rounded items-center pointer-cursor justify-between hover:bg-zinc-100"
>
	<button on:click class="w-full text-left"><slot /></button>
</DropdownMenu.Item>
