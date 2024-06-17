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
	 * The data that is passed to the component
	 * @type {Object}
	 */
	export let data = {};
	/**
	 * The wasm URL where the WIT Component can be resolved
	 */
	export let wasmURL;

	/**
	 * The callback function that is passed to the component
	 * @type {Function}
	 */
	export let eventHandler = (payload) => {
		dispatch('event', payload);
	};

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
			console.log('component', component);
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

		console.log('importables', importables);

		// load the import handles into the Wasm component and get the ES module returned
		wurbo = new Wurbo({ arrayBuffer: wasmBytes, importables, templates }, eventHandler);

		renderedHTML = await wurbo.render({
			// technically `all-content` could be named anything, but convention for now is to use `all-content`
			tag: 'all-content',
			val: data
		});
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
