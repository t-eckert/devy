<script lang="ts">
	import { DropdownMenu } from "bits-ui"
	export let href = "#"
</script>

<DropdownMenu.Item
	class="text-sm mx-1 px-2 py-0.5 group flex flex-row rounded items-center pointer-cursor justify-between hover:bg-stone-100"
>
	<a {href} class="w-full"><slot /></a>
</DropdownMenu.Item>
