<script>
	import { page } from "$app/stores"
</script>

<div class="w-full flex items-center justify-center">
	<div class="my-12">
		<div>Error</div>
		<div class="text-6xl">{$page.status}</div>
		<a class="underline" href="/">Return home</a>
	</div>
</div>
