<script lang="ts">
	import Card from "$lib/components/card/card.svelte"
	import { ArrowLeft, ArrowTopRight } from "radix-icons-svelte"
	import { PUBLIC_API } from "$env/static/public"
	import Link from "$lib/elements/link.svelte"
</script>

<div class="flex flex-col gap-2">
	<div>
		<h1 class="font-medium text-xl">Login</h1>
	</div>
	<Card>
		<div class="p-6 flex flex-row gap-8">
			<div class="w-72">
				<p class="text-sm mb-2">
					Before you can create a blog, you need to <Link href={`${PUBLIC_API}/auth/login`}
						>log in with GitHub</Link
					>.
				</p>
			</div>
		</div>
	</Card>
	<div class="flex justify-between items-center gap-2">
		<a
			href="/"
			class="flex flex-row group items-center gap-1 font-medium rounded px-2 py-1 text-sm"
		>
			<ArrowLeft class="w-4 h-4 group-hover:-translate-x-1 transition" />
			<span>Back</span></a
		>

		<a
			href={`${PUBLIC_API}/auth/login`}
			class="flex flex-row group items-center gap-1 font-medium rounded text-zinc-700 px-2 py-1 text-sm"
		>
			<span>Login</span>
			<ArrowTopRight
				class="w-4 h-4 group-hover:translate-x-1 group-hover:-translate-y-1 transition"
			/>
		</a>
	</div>
</div>
