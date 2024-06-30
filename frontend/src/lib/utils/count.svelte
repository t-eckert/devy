<script lang="ts">
	export let count = 0
	let display = count.toString()

	if (count > 999) {
		display = Math.floor(count / 1000) + "k"
	}
</script>

{display}
