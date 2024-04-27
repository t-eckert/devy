<script lang="ts">
	import { ArrowRight } from "radix-icons-svelte"
	import Card from "$lib/components/card/card.svelte"
	import Link from "$lib/elements/link.svelte"
</script>

<section class="flex flex-col gap-4 text-sm">
	<Card styling="p-2 border border-stone-200 dark:border-stone-700">
		<h2 class="font-medium text-xl mb-1">Welcome to Devy!</h2>
		<p class="mb-2">Devy is a blogging platform powered by GitHub and Markdown.</p>
		<p>
			Create a blog from a repo of markdown files and <code
				class="bg-stone-200 px-0.5 dark:bg-stone-700">git push</code
			> to publish.
		</p>
		<a href="/new/blog" class="mt-8 flex flex-row gap-1 items-center group"
			><span class="font-medium">Create your blog</span><ArrowRight
				class="group-hover:translate-x-1 transition-all"
			/>
		</a>
	</Card>
	<Card
		styling="p-2 bg-orange-100 text-orange-900 border-orange-200 dark:bg-orange-950 dark:text-orange-300 dark:border-orange-800 flex flex-col gap-2"
	>
		<p>
			Devy is still in Preview. Thank you for checking it out while it's still in its early stages.
		</p>
		<p>
			Sharing your <a href="/feedback" class="underline font-medium">feedback</a> is an amazing way you
			can help make this site better.
		</p>
	</Card>
</section>
