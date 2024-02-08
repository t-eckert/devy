<script lang="ts">
	import Home from "./home.svelte"
	import User from "./user.svelte"
	import ThemeSwitcher from "./theme-switcher.svelte"
	import Nav from "./nav.svelte"
</script>

<header class="">
	<div class="mx-auto max-w-6xl px-2 py-3 flex flex-row justify-between items-center">
		<Home />
		<div class="flex flex-row gap-1">
			<User />
			<Nav />
		</div>
	</div>
</header>
