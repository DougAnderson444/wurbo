<script>
	import { onMount, tick, createEventDispatcher } from 'svelte';
	import { Wurbo, wurboIn } from 'wurbo';
	import { $init as init, provider } from './loader/graph/graph.js';

	const dispatch = createEventDispatcher();

	/**
	 * The templates that are passed to the component
	 * @type {Array<[string, string]>}
	 */
	export let templates = null;

	/**
	 * The data that is passed to the wasm component. Needs to match the WIT, ie.
	 * data = {
	 *	  tag: 'all-content',
	 *		val: valuesHere
	 * }
	 * @type {Object} - Default: null
	 */
	export let data = {
		tag: 'all-content',
		val: null
	};
	/**
	 * The wasm URL where the WIT Component can be resolved
	 * @type {string} - Default: ''
	 */
	export let wasmURL;

	/**
	 * The callback function that is passed to the component
	 * @type {Function} - Default: null
	 */
	export let eventHandler = (payload) => {
		dispatch('event', payload);
	};

	/**
	 * Whether the component should be rendered inline or not. This is only needed
	 * if Wurbo is going to be bundled into a dataurl like in `integrity-wallet`.
	 * @type {boolean} - whether wurbo worker is inlined - Default: false
	 */
	export let inline = false;

	/**
	 * The rendered component as a string of HTML
	 * @type {string | null} - Default: null
	 */
	let renderedHTML;

	/**
	 * The module that loads the WebAssembly component
	 *
	 * @type {{ render: (arg0: string) => string | null; listen: () => void; }}
	 */
	let wurbo;

	// The Component graph, to get the name of the wasm component
	let Graph = null;

	onMount(async () => {
		init.then(() => (Graph = new provider.Graph()));

		// get your wasm bytes from your storage source
		let wasmBytes = await fetch(wasmURL).then((res) => res.arrayBuffer());

		let bytes = new Uint8Array(wasmBytes);

		// use Graph to get the importables from the bytes
		let imports;
		try {
			let component = Graph.addComponent('loaded', bytes);
			imports = component.imports;
		} catch (error) {
			console.error('error getting importables', { error });
		}

		let importables = imports
			.filter((i) => i.name.includes('/wurbo-in'))
			.map((i) => {
				let name = i.name.split('@')[0];
				return { [name]: wurboIn };
			});

		// load the import handles into the Wasm component and get the ES module returned
		wurbo = new Wurbo({ arrayBuffer: wasmBytes, importables, templates, inline }, eventHandler);
	});

	// Once the HTML is rendered and the module is loaded, we can activate the event emitters
	$: if (data && wurbo)
		wurbo.render(data).then(async (html) => {
			console.log('data val', data);
			renderedHTML = html;
		});

	// Activate the event listeners
	$: if (renderedHTML && wurbo)
		tick().then(() => {
			wurbo.activate();
		});
</script>

<div>
	{#if renderedHTML}
		{@html renderedHTML}
	{/if}
</div>
