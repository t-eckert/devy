<script lang="ts">
	import ThemeToggle from "$lib/components/theme-toggle.svelte"
	import Wrapper from "./wrapper.svelte"

	import { getTheme } from "$lib/state/theme.svelte"

	const themeState = getTheme()
</script>

<Wrapper title="Theme State">
	<div class="w-full flex flex-row justify-between items-center dark:text-white">
		<span class="capitalize text-sm">
			{themeState.theme}
		</span>
		<ThemeToggle />
	</div>
</Wrapper>
