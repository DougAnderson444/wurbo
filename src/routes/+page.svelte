<script>
	import { onMount, tick } from 'svelte';

	// Import wasm component bytes as a url
  import wasmURL from "../../target/wasm32-wasi/release/vowels.wasm?url";

	// get imports as a string
	import importableCode from './importables.js?raw';

	/**
	 * The rendered component as a string of HTML
   * @type {string | null}
	 */
	let whatSayYou;
	/**
	 * The module that loads the WebAssembly component
   * 
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

		let importables = [{'demo:vowels/imports': importableCode}];

		mod = await load( wasmBytes, importables );

		// @ts-ignore
		whatSayYou = mod.render('World');
		// @ts-ignore
		window.wurbo =  { "render": mod.render }; // expose render function to blob URLs

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

	// need to apply listeners every time the DOM renders!
	$: if (whatSayYou && mod)
		(async () => {
			// wait for the DOM to reflect the updates first
			await tick();
			mod.listen();
      console.log(`listening timestamp ${Date.now()}`);
		})();
</script>

<div>
	{#if whatSayYou}
		{@html whatSayYou}
	{/if}
</div>
