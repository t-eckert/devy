<script context="module">
	import Accordian from "./accordian.svelte"
	import AccordianItem from "./accordian-item.svelte"
	import AccordianItemHeader from "./accordian-item-header.svelte"
	import AccordianItemBody from "./accordian-item-body.svelte"
	import AccordianHeader from "./accordian-header.svelte"

	export const meta = {
		title: "components/Accordian",
		component: Accordian
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template>
	<div class="w-96">
		<Accordian>
			<AccordianHeader slot="header"
				><h1 class="font-semibold">Systems Programming Languages</h1></AccordianHeader
			>
			<AccordianItem>
				<AccordianItemHeader slot="item-header">C++</AccordianItemHeader>
				<AccordianItemBody slot="item-body">
					<p>
						A general-purpose programming language created by Bjarne Stroustrup as an extension of
						the C programming language, or "C with Classes".
					</p>
				</AccordianItemBody>
			</AccordianItem>
			<AccordianItem>
				<AccordianItemHeader slot="item-header">Go</AccordianItemHeader>
				<AccordianItemBody slot="item-body">
					<p>
						It's a statically typed, compiled language designed at Google by Robert Griesemer, Rob
						Pike, and Ken Thompson.
					</p>
				</AccordianItemBody>
			</AccordianItem>
			<AccordianItem>
				<AccordianItemHeader slot="item-header">Rust</AccordianItemHeader>
				<AccordianItemBody slot="item-body">
					<p>
						It's a systems programming language that runs blazingly fast, prevents segfaults, and
						guarantees thread safety.
					</p>
				</AccordianItemBody>
			</AccordianItem>
		</Accordian>
	</div>
</Template>

<Story name="Systems Programming Languages"></Story>
