<script lang="ts">
	import Codeblock from "$lib/components/codeblock.svelte"

	const lang = "python"
	const code = $state(`def hello():
	print("Hello world!")
`)
</script>

<Codeblock {lang} {code} />
