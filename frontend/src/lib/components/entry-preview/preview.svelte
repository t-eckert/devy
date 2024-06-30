<script lang="ts">
	import Card from "$lib/components/card/card.svelte"
</script>

<Card>
	<div class="flex flex-row">
		<div class="flex flex-col items-center justify-center">
			<slot name="likes" />
		</div>
		<div class="flex flex-col">
			<slot name="blog" />
			<slot name="title" />
			<div class="flex flex-row gap-1 items-center flex-wrap">
				<slot name="author" />
				<slot name="date" />
				<slot name="bookmark" />
			</div>
		</div>
	</div>
</Card>
