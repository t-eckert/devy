<script lang="ts">
	const { data } = $props()
	const { users } = data
</script>

<div class="flex flex-col">
	{#each users as user}
		<div>
			<span>{user.username}</span>
		</div>
	{/each}
</div>
