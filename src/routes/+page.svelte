<script>
	import { onMount, tick } from 'svelte';
	import { browser } from '$app/environment';

	// @ts-ignore
	// Import wasm component bytes as a url
  import wasmURL from "../../target/wasm32-wasi/release/vowels.wasm?url";

	// get imports as a string
	import importableCode from './importables.js?raw';

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
	/**
	 * @type {{ render: (arg0: string) => string | null; listen: () => void; }}
	 */
	let mod;

	onMount(async () => {
		// Set up a broadcast channel to listen for updates from the Blob URLs
		const bc = new BroadcastChannel('listener_updates');

		// @ts-ignore
		const { load } = await import('rollup-plugin-wit-component');

		// get wasm bytes from url
		let wasmBytes = await fetch(wasmURL).then((res) => res.arrayBuffer());

		let importables = [{'wurbo:vowels/imports': importableCode}];

    console.log({importables});

		mod = await load( wasmBytes, importables );

		// @ts-ignore
		whatSayYou = mod.render('World');
		// @ts-ignore
		window.render = mod.render; // expose render function to blob URLs
		// await tick(); // wait for the DOM to be updated with the new Elements

		// Listen for messages from the Blob URLs created in wasmComponentBytesToESModule
		bc.onmessage = (event) => {
			// create an HTMLElement from the string, then extract the top most element id from the HTMLElement
			let id =
				new DOMParser().parseFromString(event.data || '', 'text/html')?.body?.firstElementChild
					?.id || null;
			// if the id is not null, then we can update the html with the new string
			if (id) {
				// @ts-ignore
				let chosen = document.getElementById(id);
				if (chosen) {
					// @ts-ignore
					chosen.outerHTML = event.data;
					return;
				}
			}
			// else, replace all the html with the new string
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
