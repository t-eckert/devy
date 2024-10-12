<script lang="ts">
	import { getTheme } from "$lib/state/theme.svelte"

	import Sun from "$lib/icons/sun.svelte"
	import Moon from "$lib/icons/moon.svelte"

	const themeState = getTheme()
</script>

<button
	class={`relative inline-flex h-6 w-12 flex-shrink-0 cursor-pointer rounded-md border-2 border-transparent ` +
		`transition-colors duration-200 ease-in-out ` +
		`focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2 ` +
		`${themeState.theme === "light" ? "bg-stone-200" : "bg-zinc-900"}`}
	onclick={() => {
		themeState.toggle()
	}}
>
	<span
		aria-hidden="true"
		class={`pointer-events-none inline-block absolute h-5 w-5 translate-x-0 transform rounded shadow ring-0 transition ` +
			`duration-200 ease-in-out ` +
			`${themeState.theme === "light" ? "bg-white" : "bg-zinc-700 translate-x-6"}`}
	>
	</span>
	<span class="z-10 flex flex-row items-center h-full px-0.5 gap-2">
		<div class="h-4 w-4">
			<Sun />
		</div>
		<div class="h-4 w-4">
			<Moon />
		</div>
	</span>
</button>
