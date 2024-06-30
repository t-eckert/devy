<script lang="ts">
	import Home from "./home.svelte"
	import UserToken from "./user-token.svelte"
	import Nav from "./nav.svelte"
</script>

<header class="">
	<div class="mx-auto max-w-6xl px-2 py-3 flex flex-row justify-between items-center">
		<Home />
		<div class="flex flex-row gap-1">
			<UserToken />
			<Nav />
		</div>
	</div>
</header>
