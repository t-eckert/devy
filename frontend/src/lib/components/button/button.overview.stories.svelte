<script context="module">
	import Button from "./button.svelte"
	export const meta = {
		title: "components/Button",
		component: Button
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"
</script>

<Template>
	<div class="grid grid-cols-4 grid-rows-4 gap-4 items-center justify-items-center">
		<div class="col-start-1 row-start-2 font-semibold">Primary</div>
		<div class="col-start-1 row-start-3 font-semibold">Secondary</div>
		<div class="col-start-1 row-start-4 font-semibold">Tertiary</div>

		<div class="col-start-2 row-start-1 font-semibold">Neutral</div>
		<div class="col-start-3 row-start-1 font-semibold">Positive</div>
		<div class="col-start-4 row-start-1 font-semibold">Negative</div>

		<div class="col-start-2 row-start-2">
			<Button role="primary" behavior="neutral">
				<span class="text-sm font-medium">Next action</span>
			</Button>
		</div>

		<div class="col-start-3 row-start-2">
			<Button role="primary" behavior="positive">
				<span class="text-sm font-medium">Create blog</span>
			</Button>
		</div>

		<div class="col-start-4 row-start-2">
			<Button role="primary" behavior="negative">
				<span class="text-sm font-medium">Delete post</span>
			</Button>
		</div>

		<div class="col-start-2 row-start-3">
			<Button role="secondary" behavior="neutral">
				<span class="text-sm font-medium">Next action</span>
			</Button>
		</div>

		<div class="col-start-3 row-start-3">
			<Button role="secondary" behavior="positive">
				<span class="text-sm font-medium">Create blog</span>
			</Button>
		</div>

		<div class="col-start-4 row-start-3">
			<Button role="secondary" behavior="negative">
				<span class="text-sm font-medium">Delete post</span>
			</Button>
		</div>

		<div class="col-start-2 row-start-4">
			<Button role="tertiary" behavior="neutral">
				<span class="text-sm font-medium">Next action</span>
			</Button>
		</div>

		<div class="col-start-3 row-start-4">
			<Button role="tertiary" behavior="positive">
				<span class="text-sm font-medium">Create blog</span>
			</Button>
		</div>

		<div class="col-start-4 row-start-4">
			<Button role="tertiary" behavior="negative">
				<span class="text-sm font-medium">Delete post</span>
			</Button>
		</div>
	</div>
</Template>

<Story name="Overview"></Story>
