<script>
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let examples;

	async function browserLoad() {
		if (!import.meta.env.SSR && browser) {
			// This code will only run in the browser
			let ex = await import('$examples/index.js');
			examples = ex.default;
		} else {
			// This code will only run in the server
			console.log('server', browser);
		}
	}
	browserLoad();
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
