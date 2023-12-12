<script>
	import { onMount, tick } from 'svelte';
	import * as wurbo from 'wurbo';

	// Import wasm component bytes as a url
	import wasmURL from '../../../../target/wasm32-wasi/release/vowels.wasm?url';

	// get imports from +page.svelte
	export let importableCode;
	export let load;

	/**
	 * The rendered component as a string of HTML
	 * @type {string | null}
	 */
	let renderedHTML;
	/**
	 * The module that loads the WebAssembly component
	 *
	 * @type {{ render: (arg0: string) => string | null; listen: () => void; }}
	 */
	let mod;

	onMount(async () => {
		// ensure you are in the Browser environment to rollup your WIT Component
		// const { load } = await import('rollup-plugin-wit-component');

		let listener = new wurbo.Listener();

		importableCode = `
      export function addeventlistener({ selector, ty, outputid, template }) {
        const bc = new BroadcastChannel('listener_updates');
        document.querySelector(selector).addEventListener(ty, (e) => {
          bc.postMessage(window.${listener.namespace}.render({
            tag: 'output',
            val: {
              value: e.target.value,
              id: outputid,
              template
            }
          }));
        });
      }`;

		// get your wasm bytes from your storage source
		let wasmBytes = await fetch(wasmURL).then((res) => res.arrayBuffer());

		// define the import handles you are giving to your component
		let importables = [{ 'demo:vowels/wurbo-in': importableCode }];

		// load the import handles into the Wasm component and get the ES module returned
		mod = await load(wasmBytes, importables);

		// call `render` with your inputs for the component
		let data = {
			tag: 'content',
			val: {
				page: { title: "Let's count vowels using templates for Inputs and Outputs!" },
				input: { placeholder: "Input the word with vowels it's here" },
				output: { value: 'vowels' }
			}
		};
		renderedHTML = mod.wurboOut.render(data);

		// lisen for events from the component
		listener.listen(mod);
	});

	// Once the HTML is rendered and the module is loaded, we can activate the event emitters
	$: if (renderedHTML && mod)
		(async () => {
			// wait for the DOM to reflect the updates first
			await tick();
			// once the DOM has our elements loaded, we can activate the event emitters
			mod.wurboOut.activate();
		})();
</script>

<div>
	{#if renderedHTML}
		{@html renderedHTML}
	{/if}
</div>
