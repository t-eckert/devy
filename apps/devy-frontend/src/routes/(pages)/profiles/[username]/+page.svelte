<script lang="ts">
	import Column from "$lib/layouts/column.svelte"
	import Entry from "$lib/components/entry.svelte"
	import Avatar from "$lib/components/avatar.svelte"
	import Card from "$lib/components/card.svelte"

	import Json from "$lib/utils/json.svelte"
	import RelativeDate from "$lib/utils/relative-date.svelte"

	const { data } = $props()
	const { profile, user, blogs, entries } = data

	let showData = false
</script>

<svelte:head>
	<title>{profile.displayName}</title>
</svelte:head>

{#if showData}
	<section class="my-3 w-full rounded shadow">
		<Json {data} />
	</section>
{/if}

<section class="grid grid-cols-1 gap-6 pt-3 sm:grid-cols-4">
	<div class="flex flex-col gap-3">
		<div class="aspect-square rounded-xl">
			<Avatar src={profile.avatarUrl} displayName={profile.displayName} />
		</div>
		<ul class="flex flex-col gap-1 text-sm">
			{#if profile.githubUsername}
				<li>
					<a
						href={`https://github.com/${profile.githubUsername}`}
						class="group flex flex-row gap-1"
					>
						<span class="font-medium text-slate-800 group-hover:text-slate-950">GitHub</span>
						<span class="text-slate-600 group-hover:text-slate-800">{profile.githubUsername}</span>
					</a>
				</li>
			{/if}
			{#if profile.websiteUrl}
				<li>
					<a href={profile.websiteUrl} class="group flex flex-row gap-1">
						<span class="font-medium text-slate-800 group-hover:text-slate-950">Website</span>
						<span class="text-slate-600 group-hover:text-slate-800"
							>{profile.websiteUrl.split("//")[1]}</span
						>
					</a>
				</li>
			{/if}
			{#if profile.twitterUsername}
				<li>
					<a href={`https://x.com/${profile.twitterUsername}`} class="group flex flex-row gap-1">
						<span class="font-medium text-slate-800 group-hover:text-slate-950">Twitter</span>
						<span class="text-slate-600 group-hover:text-slate-800">{profile.twitterUsername}</span>
					</a>
				</li>
			{/if}
		</ul>
		<div class="text-sm text-slate-800">User since <RelativeDate date={user.createdAt} /></div>
	</div>

	<div class="col-span-2 flex flex-col items-start gap-6">
		<div>
			<h1
				class="-mt-4 break-normal font-serif text-3xl font-medium text-zinc-800 sm:text-7xl sm:leading-tight"
			>
				{profile.displayName}
			</h1>

			<p class="w-full max-w-xl">
				{profile.bio}
			</p>
		</div>

		<div class="flex w-full flex-col gap-3">
			{#each entries as entry}
				<Entry {entry} omit={["author"]} />
			{/each}
		</div>
	</div>
</section>
