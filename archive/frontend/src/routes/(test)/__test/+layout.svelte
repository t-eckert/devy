<script lang="ts">
	import type { LayoutData } from "./$types"
	import Controls from "./controls.svelte"

	export let data: LayoutData
</script>

<div class="w-[100vw] h-[100vh] flex flex-col gap-12 items-center justify-center">
	<slot />
	<Controls tests={data.props.tests} />
</div>
