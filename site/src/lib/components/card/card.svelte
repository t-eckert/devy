<script lang="ts">
	export let styling = ""
</script>

<div
	class={[
		"rounded-md border backdrop-blur-lg shadow",
		styling || "border-stone-100 bg-zinc-50 dark:bg-zinc-900 dark:border-stone-200"
	].join(" ")}
>
	<slot />
</div>
