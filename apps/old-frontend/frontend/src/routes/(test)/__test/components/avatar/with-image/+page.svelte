<script lang="ts">
	import Avatar from "$lib/components/avatar.svelte"

	const props = {
		src: "https://images.unsplash.com/photo-1513152697235-fe74c283646a",
		displayName: "Tobias Eckert"
	}
</script>

<Avatar {...props} />
