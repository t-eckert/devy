<script lang="ts">
	import { onMount } from "svelte"

	let dragging = false
	let x = 50
	let y = 50
	let startX: number
	let startY: number

	// Function to start dragging
	function dragStart(event: any) {
		dragging = true
		startX = event.clientX - x
		startY = event.clientY - y
	}

	// Function to stop dragging
	function dragStop() {
		dragging = false
	}

	// Function to handle dragging
	function draggingFunction(event: any) {
		if (dragging) {
			x = event.clientX - startX
			y = event.clientY - startY
		}
	}

	// Attach mousemove and mouseup event listeners to the window
	onMount(() => {
		window.addEventListener("mousemove", draggingFunction)
		window.addEventListener("mouseup", dragStop)

		return () => {
			window.removeEventListener("mousemove", draggingFunction)
			window.removeEventListener("mouseup", dragStop)
		}
	})

	export let title = "Window"
	export let expanded = false

	$: expanded
</script>

<section
	class="fixed z-50 rounded-xl shadow-xl bg-white/20 backdrop-blur-lg border"
	style="left: {x}px; top: {y}px;"
>
	<div
		class="px-2 min-w-44 rounded-t-xl h-8 flex flex-row items-center justify-between cursor-move"
		on:mousedown={dragStart}
	>
		<h1 class="text-xs font-medium">{title}</h1>
		<button class="text-xs font-mono" on:click={() => (expanded = !expanded)}>
			{expanded ? "[min]" : "[exp]"}
		</button>
	</div>
	{#if expanded}
		<div class="p-1 border-t">
			<slot />
		</div>
	{/if}
</section>
