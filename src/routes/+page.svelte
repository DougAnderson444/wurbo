<script>
	import { onMount, tick } from 'svelte';
  import * as wurbo from 'wurbo';

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
    // ensure you are in the Browser environment to rollup your WIT Component
		const { load } = await import('rollup-plugin-wit-component');

		// get your wasm bytes from your storage source
		let wasmBytes = await fetch(wasmURL).then((res) => res.arrayBuffer());

    // define the import handles you are giving to your component
		let importables = [{'demo:vowels/imports': importableCode}];

    // load the import handles into the Wasm component and get the ES module returned
		mod = await load( wasmBytes, importables );

		// call `render` with your inputs for the component
		whatSayYou = mod.render('Worldz');

    // lisen for events from the component 
    wurbo.listen(mod);

	});

	// need to apply listeners every time the DOM renders!
	$: if (whatSayYou && mod)
		(async () => {
			// wait for the DOM to reflect the updates first
			await tick();
      // once the DOM has our elements loaded, we can activate the event emitters
			mod.activate();
      console.log(`listening timestamp ${Date.now()}`);
		})();
</script>

<div>
	{#if whatSayYou}
		{@html whatSayYou}
	{/if}
</div>
