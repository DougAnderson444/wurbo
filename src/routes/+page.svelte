<script>
	import { onMount, tick } from 'svelte';
  import { setup } from 'wurbo';

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

    setup(mod);

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
