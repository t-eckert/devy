<script lang="ts">
	export let href: string
	export let title: string = ""
	export let text: string
</script>

<img src={href} {title} alt={text} class="rounded" />
