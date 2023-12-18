<script>
	import { onMount } from 'svelte';
	import * as wurbo from 'wurbo';
	import examples from '$examples/index.js';

	// get imports as a string
	import importableCode from './importables.js?raw';

	import { buildCodeString } from './importables.js';

	let load;

	onMount(async () => {
		// ensure you are in the Browser environment to rollup your WIT Component
		({ load } = await import('rollup-plugin-wit-component'));
	});
</script>

<div>
	<!-- For each Svelte file in examples, display it -->
	{#if load}
		{#each examples as example}
			<div>
				<svelte:component this={example} {load} {buildCodeString} />
			</div>
		{/each}
	{/if}
</div>
