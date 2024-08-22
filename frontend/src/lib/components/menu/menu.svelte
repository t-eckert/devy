<script lang="ts">
	import HamburgerMenu from "$lib/icons/hamburger-menu.svelte"

	let isOpen = $state(false)
	let { children } = $props()

	function clickOutside(node: any) {
		const handleClick = (event: any) => {
			if (node && !node.contains(event.target) && !event.defaultPrevented) {
				node.dispatchEvent(new CustomEvent("click_outside", node))
			}
		}

		document.addEventListener("click", handleClick, true)

		return {
			destroy() {
				document.removeEventListener("click", handleClick, true)
			}
		}
	}

	const toggleMenu = (e: any) => {
		e.stopPropagation()
		isOpen = !isOpen
	}
</script>

<div class="relative">
	<button
		onclick={toggleMenu}
		class={`p-1 rounded-md text-stone-600 dark:text-zinc-300 hover:text-stone-950 dark:hover:text-zinc-100 transition-all border ` +
			`${isOpen ? "bg-white border-stone-200/70 shadow dark:shadow-none dark:bg-zinc-800 dark:border-zinc-600/70" : "bg-none border-transparent hover:bg-stone-100 dark:hover:bg-zinc-800"}`}
	>
		<HamburgerMenu />
	</button>

	{#if isOpen}
		<div class="absolute mt-1 right-0 z-50">
			<section
				class="h-full w-40 rounded-md border border-stone-200/70 dark:border-zinc-700/70 backdrop-blur-3xl py-1 flex flex-col gap-1 shadow bg-white/60 dark:bg-zinc-900 dark:border-stone-200"
			>
				{@render children()}
			</section>
		</div>
	{/if}
</div>
