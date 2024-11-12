<script lang="ts">
	export let href: string

	let additionalClasses = ""
	export { additionalClasses as class }
</script>

<a {href} class={["underline text-stone-700", additionalClasses].join(" ")}><slot /></a>
