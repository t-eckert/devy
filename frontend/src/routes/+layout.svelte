<script lang="ts">
	import "../app.css"
	import "@fontsource-variable/inter"
	import "@fontsource/space-mono"

	let theme = $state("light")
	let { children } = $props()
</script>

<div class={theme}>
	<!-- t-eckert: I don't know how to render the children for a layout -->
	<!-- svelte-ignore slot_element_deprecated -->
	<div class="bg-stone-50 text-stone-950 min-h-screen">
		<slot />
	</div>
</div>
