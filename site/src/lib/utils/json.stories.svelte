<script context="module">
	import Json from './json.svelte';

	export const meta = {
		title: 'Json',
		component: Json
	};
</script>

<script>
	import { Story, Template } from '@storybook/addon-svelte-csf';
</script>

<Template let:args>
	<Json {...args} />
</Template>

<Story name="Empty" />

<Story name="Pizza Order" args={{ data: { pizza: 'pepperoni' } }} />
