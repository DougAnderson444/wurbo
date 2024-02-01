<script>
	import { onMount, tick } from 'svelte';
	import { Wurbo, wurboIn } from 'wurbo';

	// Import wasm component bytes as a url
	import wasmURL from '../../../target/wasm32-wasi/debug/forms.wasm?url';

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
	let wurbo;

	onMount(async () => {
		// get your wasm bytes from your storage source
		let wasmBytes = await fetch(wasmURL).then((res) => res.arrayBuffer());

		// define the import handles you are giving to your component
		let importables = [{ 'demo:forms/wurbo-in': wurboIn }];

		// load the import handles into the Wasm component and get the ES module returned
		wurbo = new Wurbo({ arrayBuffer: wasmBytes, importables });

		// call `render` with your inputs for the component
		let data = {
			tag: 'all-content',
			val: {
				page: { title: "Let's process the contents of a form." },
				input: { placeholder: 'Enter a Username here' }
			}
		};
		renderedHTML = await wurbo.render(data);
	});

	// Once the HTML is rendered and the module is loaded, we can activate the event emitters
	$: if (renderedHTML && wurbo)
		(async () => {
			// wait for the DOM to reflect the updates first
			await tick();
			// once the DOM has our elements loaded, we can activate the event emitters
			wurbo.activate();
		})();
</script>

<div>
	{#if renderedHTML}
		{@html renderedHTML}
	{/if}
</div>
