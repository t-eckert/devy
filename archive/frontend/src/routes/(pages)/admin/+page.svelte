<script lang="ts">
	import Card from "$lib/components/card.svelte"
	import H1 from "$lib/elements/h1.svelte"

	const { data } = $props()
	const { users } = data
</script>

<svelte:head>
	<title>Admin / Overview</title>
</svelte:head>

<section class="flex flex-col gap-3">
	<H1>Overview</H1>

	<div class="flex flex-row gap-3 mt-6">
		<div class="w-24 aspect-square">
			<Card>
				<div class="h-full w-full flex flex-col items-center justify-center">
					<div class="font-mono text-4xl">{users.length}</div>
					<div class="font-medium">Users</div>
				</div>
			</Card>
		</div>
	</div>
</section>
