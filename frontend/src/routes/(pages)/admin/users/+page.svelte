<script lang="ts">
	import RelativeDate from "$lib/utils/relative-date.svelte"
	import H1 from "$lib/elements/h1.svelte"

	const { data } = $props()
	const { users } = data
</script>

<svelte:head>
	<title>Admin / Users</title>
</svelte:head>

<section class="w-full">
	<H1>Users</H1>
	<div class="flex flex-col mt-6 w-full">
		<div class="bg-white rounded border border-stone-100 shadow w-full">
			<table class="w-full">
				<thead>
					<tr class="text-sm font-medium text-stone-800 bg-stone-50">
						<th class="text-left py-1 pl-2 rounded-tl">Username</th>
						<th class="text-left py-1">Role</th>
						<th class="text-left py-1">Status</th>
						<th class="text-right py-1">Created</th>
						<th class="text-right py-1 pr-2 rounded-tr">Last Logged In</th>
					</tr>
				</thead>
				<tbody class="text-sm divide-y divide-stone-100">
					{#each users as user}
						<tr>
							<td class="text-left py-0.5 pl-2">
								<a href={`/profiles/${user.username}`} class="font-medium text-700 hover:text-950">
									{user.username}
								</a>
							</td>
							<td>{user.role}</td>
							<td>{user.status}</td>
							<td class="text-right"><RelativeDate date={user.createdAt} /></td>
							<td class="text-right pr-2"><RelativeDate date={user.updatedAt} /></td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</section>
