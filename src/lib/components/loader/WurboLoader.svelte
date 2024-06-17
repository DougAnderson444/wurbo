<script>
	// a component to send data to a plugin
	import { onMount, tick, createEventDispatcher } from 'svelte';
	import { Wurbo, wurboIn } from '../../index.js';
	import { $init as init, provider } from './graph/graph.js';
	import Progress from './Progress.svelte';

	/**
	 * Whether the worker should be loaded inline as base64 or not.
	 * Default is false.
	 */
	export let inline = false;

	let pluginFile;

	const files = new Map();
	let fileinput;
	let name;

	const dispatch = createEventDispatcher();
	/**
	 * The rendered component as a string of HTML
	 * @type {string | null}
	 */
	let renderedHTML;
	/**
	 * The module that loads the WebAssembly component
	 *
	 * @type
	 */
	let wurbo;
	/**
	 * The location hash frag
	 **/
	let api;

	let Graph = null;

	onMount(async () => {
		// init the graph utility
		init.then(() => (Graph = new provider.Graph()));
		api = location.hash.substring(1);
	});

	$: if (pluginFile) {
		// name without the .wasm on the end, because we will also store our data here
		let path = ['apps', pluginFile.name.replace(/\.wasm$/, ''), 'wasm'];
		load(pluginFile.bytes).catch((error) => {
			console.error('error loading plugin', { error });
		});
	}

	const onFileSelected = (e) => {
		let plugin = e.target.files[0];
		let reader = new FileReader();

		files.set(reader, plugin);

		reader.addEventListener('loadend', (evt) => {
			// reader.result contains the contents of blob as a typed array
			pluginFile = { bytes: reader.result, name };
		});

		reader.addEventListener('load', (evt) => {
			const file = files.get(evt.target);
			name = file.name;
		});

		// reader.readAsDataURL(plugin);
		reader.readAsArrayBuffer(plugin);
		// reader.readAsText(plugin);
	};

	// OnFileLoaded: Once the wasm bytes are loaded, we can setup the plugin
	// key is a cid string
	async function load(arrayBuffer) {
		let bytes = new Uint8Array(arrayBuffer);

		// use Graph to get the importables from the bytes
		let imports;
		try {
			let component = Graph.addComponent('loaded', bytes);
			imports = component.imports;
		} catch (error) {
			console.error('error getting importables', { error });
		}

		// filter to select any that contains `/wurbo-in`, set the importable to the `wurboIn` function,
		let importables = imports
			.filter((i) => i.name.includes('/wurbo-in'))
			.map((i) => {
				// trim string after the @
				let name = i.name.split('@')[0];
				return { [name]: wurboIn };
			});

		// load the import handles into the Wasm component and get the ES module returned
		wurbo = new Wurbo({ arrayBuffer, importables, inline }, async (payload) => {
			// Relay emitted commands from the Wasm component to PiperNet
			console.log('Command emitted: ', { payload });
			try {
				dispatch('command', payload);
				// return await piper.command(payload);
			} catch (error) {
				// it's ok to fail silently, not all messages are commands
			}
		});

		// call `render` with your inputs for the component
		renderedHTML = await wurbo.render({
			tag: 'all-content',
			val: { api }
		});

		if (pluginFile) pluginFile = null; // reset loader state
		if (fileinput) fileinput.value = null; // reset file input
	}

	// Once the HTML is rendered and the module is loaded, we can activate the event emitters
	$: if (renderedHTML && wurbo)
		(async () => {
			// wait for the DOM to reflect the updates first
			await tick();
			// once the DOM has our elements loaded, we can activate the aggregated event emitters
			wurbo.aggregation();
		})();
</script>

{#if renderedHTML}
	{@html renderedHTML}
{:else if pluginFile}
	<Progress />
{:else}
	<input
		style="display:none"
		type="file"
		accept=".wasm"
		on:change={(e) => onFileSelected(e)}
		bind:this={fileinput}
	/>
	<div class="flex flex-col p-4">
		<div
			class="flex w-fit justify-center cursor-pointer border border-green-400 rounded-md px-4 py-2 shadow"
			on:keypress={() => {
				fileinput.click();
			}}
			on:click={() => {
				fileinput.click();
			}}
		>
			<div class="flex p-2 rounded-md shadow">Browse *.wasm file...</div>
		</div>
	</div>
{/if}

<style lang="postcss">
	@tailwind utilities;
</style>
