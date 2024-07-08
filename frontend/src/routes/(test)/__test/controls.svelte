<script lang="ts">
	import { type Tests } from './tests';
	import Card from '$lib/components/card.svelte';
	import Button from '$lib/components/button.svelte';
	import DoubleArrowDown from '$lib/icons/double-arrow-down.svelte';

	let isOpen = $state(true);

	const { tests }: { tests: Tests } = $props();
</script>

<section class={`fixed left-0 w-screen transition-all ${isOpen ? 'bottom-12' : '-bottom-64'}`}>
	<div class="flex flex-col gap-3 items-center">
		<Button
			role="tertiary"
			onclick={() => {
				isOpen = !isOpen;
			}}
		>
			<div class={`${isOpen ? 'rotate-0' : 'rotate-180'} transition-all`}>
				<DoubleArrowDown />
			</div>
		</Button>

		<div class="flex flex-row gap-3 w-full max-w-6xl">
			<div class="w-full max-w-5xl h-64">
				<Card>
					<nav class="grid grid-cols-3 gap-3">
						<div class="flex flex-col">
							<h2 class="font-semibold">Components</h2>
							{#each tests.components as test}
								<a
									href={`/__test${test.path}`}
									class="font-medium text-sm text-stone-600 hover:text-stone-950">{test.name}</a
								>
							{/each}
						</div>
						<div class="flex flex-col">
							<h2 class="font-semibold">Markdown</h2>
							{#each tests.markdown as test}
								<a
									href={`/__test${test.path}`}
									class="font-medium text-sm text-stone-600 hover:text-stone-950">{test.name}</a
								>
							{/each}
						</div>
					</nav>
				</Card>
			</div>
			<div class="w-full max-w-xs">
				<Card>
					<a href="/" class="font-medium text-sm text-stone-600 hover:text-stone-950">Home</a>
				</Card>
			</div>
		</div>
	</div>
</section>
