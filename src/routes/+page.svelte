<script>
	import { onMount, tick } from 'svelte';
	import * as wurbo from 'wurbo';
	import { examples } from '$lib';

	// get imports as a string
	import importableCode from './importables.js?raw';

	let load;

	onMount(async () => {
		// ensure you are in the Browser environment to rollup your WIT Component
		({ load } = await import('rollup-plugin-wit-component'));
	});
</script>

<div>
	<!-- For each Svelte file in examples, display it -->
	{#if load}
		{#each examples.default as example}
			<div>
				<svelte:component this={example} {load} {importableCode} />
			</div>
		{/each}
	{/if}
</div>
