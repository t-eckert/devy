<script lang="ts">
	const { href }: { href: string } = $props()
</script>

<div
	class="text-sm mx-1 px-2 py-0.5 group flex flex-row rounded items-center pointer-cursor justify-between hover:bg-stone-100"
>
	<a {href} class="w-full"><slot /></a>
</div>
