<script>
	import { onMount } from 'svelte';
	import * as wurbo from 'wurbo';
	import examples from '$examples/index.js';
</script>

<div>
	<!-- For each Svelte file in examples, display it -->
	{#if examples}
		{#each examples as example}
			<div>
				<svelte:component this={example} />
			</div>
		{/each}
	{/if}
</div>
