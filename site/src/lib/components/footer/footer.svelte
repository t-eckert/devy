<script>
	import Card from "../card/card.svelte"
</script>

<footer class="w-full transition-all py-12">
	<div class="mx-auto max-w-xs sm:max-w-6xl flex flex-row items-start">
		<Card>
			<div class="p-2 pb-12">
				<div class="flex flex-row gap-2 items-baseline mb-2">
					<a href="/" class="px-0.5 -ml-0.5 font-medium">Devy</a>
					<span
						class="text-xs font-medium px-2 py-0.5 rounded-full text-white bg-amber-600 dark:text-zinc-100 dark:bg-zinc-800 select-none"
						>Preview</span
					>
				</div>
				<div class="pt-2 flex flex-col sm:flex-row gap-12 text-sm text-zinc-800 dark:text-zinc-300">
					<section class="w-full sm:w-56 flex flex-col gap-4">
						<p>
							Devy is an open source platform for people to publish blog posts in markdown from
							their GitHub repositories.{" "}
						</p>
						<p>Thank you for checking it out.</p>
						<span class="self-end">
							&#8212;{" "}
							<a href="https://thomaseckert.dev">Thomas Eckert</a>
						</span>
					</section>
				</div>
			</div>
		</Card>
	</div>
</footer>
