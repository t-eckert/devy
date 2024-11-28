<script lang="ts">
	import { browser } from "$app/environment"
	import { page } from "$app/stores"
	import { onMount } from "svelte"
	import * as Fathom from "fathom-client"

	onMount(async () => {
		Fathom.load("SRGLORAK", {
			url: "https://cdn.usefathom.com/script.js",
			includedDomains: ["devy.page"]
		})
	})

	$: $page.url.pathname, browser && Fathom.trackPageview()
</script>
