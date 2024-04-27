<script lang="ts">
	import Main from "$lib/layouts/main.svelte"
	import Sidebar from "$lib/components/sidebar/sidebar.svelte"
</script>

<Main>
	<div class="sm:py-4 grid grid-cols-1 sm:grid-cols-4 gap-6 items-start">
		<div class="col-start-1 sm:col-span-3">
			<div class="flex flex-row gap-4">
				<div class="flex flex-col col-start-1 sm:flex-col items-start gap-2">
					<a
						href="/feeds/recent"
						class={[
							"bg-stone-50 text-sm  px-2 py-1 sm:w-52 flex flex-row items-start font-medium rounded-lg border hover:pl-4 transition-all",
							"text-stone-900 border-stone-200 shadow-xs"
						].join(" ")}>Recent</a
					>
				</div>
				<slot />
			</div>
		</div>
		<div class="hidden sm:block sm:col-span-1">
			<Sidebar />
		</div>
	</div>
</Main>
