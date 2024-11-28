<script lang="ts">
	type Style = "default" | "alert" | "warning" | "success" | "info"

	const common_classes = "px-3 py-1 rounded-t-md border"

	const styles = {
		default: "border-stone-200/70 bg-stone-100 text-stone-800",
		alert: "bg-red-50/70 text-red-600 border-red-200/70",
		warning: "bg-yellow-100 text-yellow-800",
		success: "bg-green-100 text-green-800",
		info: "bg-blue-100 text-blue-800"
	}

	export let style: Style = "default"

	let classes = `${common_classes} ${styles[style]}`
</script>

<div class={classes}>
	<slot />
</div>
