<script>
	import { onMount, tick } from 'svelte';
	import { browser } from '$app/environment';

	// @ts-ignore
	// Import wasm wasm component bytes as a url
	// Can import either wasi (if you have sys dependencies such as getrandom) or unknown-unknown (if you don't)
	// import wasmURL from '../../crates/target/wasm32-wasi/release/hello.wasm?url';
	import wasmURL from '../../crates/target/wasm32-unknown-unknown/release/hello.wasm?url';

	// get imports as a string
	import importables from './importables.js?raw';

	/**
	 * @type {string | null}
	 */
	let whatSayYou;
	/**
	 * @type {string}
	 */
	let code = 'Standby, generating your bundle...';
	/**
	 * @type {Node}
	 */
	let parentDiv;
	let mod;

	onMount(async () => {
		// Set up a broadcast channel to listen for updates from the Blob URLs
		const bc = new BroadcastChannel('listener_updates');

		const { load } = await import('rollup-plugin-wit-component');
		let wasmComponentBytesToESModule = await load();

		// get wasm bytes from url
		let wasmBytes = await fetch(wasmURL).then((res) => res.arrayBuffer());

		let importName = './importables.js';

		let imports = {
			map: {
				'component:cargo-comp': importName
			},
			files: [[importName, importables]]
		};

		mod = await wasmComponentBytesToESModule({ wasmBytes, imprt: imports });

		// @ts-ignore
		whatSayYou = mod.render('World');
		window.render = mod.render; // expose render function to blob URLs
		// await tick(); // wait for the DOM to be updated with the new Elements

		// Listen for messages from the Blob URLs created in wasmComponentBytesToESModule
		bc.onmessage = (event) => {
			whatSayYou = event.data;
		};
	});

	// need to apply listeners very time the DOM gets new render!
	$: if (whatSayYou && mod)
		(async () => {
			// wait for the DOM to reflect the updates first
			await tick();
			mod.listen();
			// put focus back on thelast focused element
		})();
</script>

<div bind:this={parentDiv}>
	{#if whatSayYou}
		{@html whatSayYou}
	{/if}
</div>
