<script lang="ts">
	import { PUBLIC_API } from "$env/static/public"
	import Json from "$lib/utils/json.svelte"
	import { onMount } from "svelte"

	import { useClerkContext } from "svelte-clerk"

	const ctx = useClerkContext()

	let p = $state("initial")

	onMount(async () => {
		const token = await ctx.session?.getToken()

		if (token) {
			console.log(token)
			const resp = await fetch(`${PUBLIC_API}/protected`, {
				headers: { Authorization: token }
			})
			console.log(resp)

			if (resp.ok) {
				p = await resp.text()
			}
		}
	})

	const { data } = $props()
</script>

<Json {data} />

<dir>{p}</dir>
