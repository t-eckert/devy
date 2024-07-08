<script lang="ts">
	import Accordian from "$lib/components/accordian/accordian.svelte"
	import AccordianHeader from "$lib/components/accordian/accordian.header.svelte"
	import AccordianItem from "$lib/components/accordian/accordian.item.svelte"
	import AccordianItemHeader from "$lib/components/accordian/accordian.item.header.svelte"
	import AccordianItemBody from "$lib/components/accordian/accordian.item.body.svelte"
</script>

<div class="w-96">
	<Accordian>
		<AccordianHeader slot="header"
			><h1 class="font-semibold">Systems Programming Languages</h1></AccordianHeader
		>
		<AccordianItem>
			<AccordianItemHeader slot="item-header">C++</AccordianItemHeader>
			<AccordianItemBody slot="item-body">
				<p>
					A general-purpose programming language created by Bjarne Stroustrup as an extension of the
					C programming language, or "C with Classes".
				</p>
			</AccordianItemBody>
		</AccordianItem>
		<AccordianItem>
			<AccordianItemHeader slot="item-header">Go</AccordianItemHeader>
			<AccordianItemBody slot="item-body">
				<p>
					A statically typed, compiled language designed at Google by Robert Griesemer, Rob Pike,
					and Ken Thompson.
				</p>
			</AccordianItemBody>
		</AccordianItem>
		<AccordianItem>
			<AccordianItemHeader slot="item-header">Rust</AccordianItemHeader>
			<AccordianItemBody slot="item-body">
				<p>
					A systems programming language that runs blazingly fast, prevents segfaults, and
					guarantees thread safety.
				</p>
			</AccordianItemBody>
		</AccordianItem>
	</Accordian>
</div>
