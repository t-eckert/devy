<script lang="ts">
	import { dev } from '$app/environment';

	import Menu from '$lib/components/menu';
</script>

<nav class="flex flex-row items-center gap-3">
	<a href="/sign-in" class="text-sm font-medium text-stone-600 hover:text-stone-950">Sign in</a>
	<a href="/feedback" class="text-sm font-medium text-stone-600 hover:text-stone-950">Feedback</a>
	<a href="/changelog" class="text-sm font-medium text-stone-600 hover:text-stone-950">Changelog</a>
	{#if dev}
		<a href="/__test/" class="text-sm font-medium text-stone-600 hover:text-stone-950">Tests</a>
	{/if}
</nav>
