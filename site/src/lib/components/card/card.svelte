<script lang="ts">
	let className = ""
	export { className as class }
</script>

<div
	class={["rounded-md border border-stone-100 bg-zinc-50 backdrop-blur-lg shadow", className].join(
		" "
	)}
>
	<slot />
</div>
