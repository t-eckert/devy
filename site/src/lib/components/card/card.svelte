<script lang="ts">
	export let styling = ""
</script>

<div
	class={[
		"rounded-md border backdrop-blur-lg shadow",
		styling || "border-stone-100 bg-zinc-50"
	].join(" ")}
>
	<slot />
</div>
