<script lang="ts">
	import Json from "$lib/utils/json.svelte"
	import Rocket from "$lib/icons/rocket.svelte"
	import Clock from "$lib/icons/clock.svelte"

	import { page } from "$app/stores"

	let feed = $derived($page.url.pathname.split("/").pop())

	const style = (id: string) =>
		[
			"font-medium px-2 py-1 border rounded-md flex flex-row items-center gap-2 sm:gap-3",
			"group transition-all",
			id === feed
				? "text-stone-950 border-stone-200 bg-white shadow"
				: "text-stone-600 border-transparent hover:text-stone-950"
		].join(" ")
</script>

<div class="flex flex-col gap-1">
	<h1 class="font-semibold text-stone-700 text-sm sr-only sm:not-sr-only">Feeds</h1>

	<div class="flex flex-row sm:flex-col gap-1">
		<a href="/feeds/popular" class={style("popular")}>
			<span class="group-hover:rotate-12 group-hover:text-red-600 transition-all"><Rocket /></span>
			<span>Popular</span>
		</a>
		<a href="/feeds/recent" class={style("recent")}>
			<span class="group-hover:rotate-12 group-hover:text-green-600 transition-all"><Clock /></span>
			<span>Recent</span>
		</a>
	</div>
</div>
