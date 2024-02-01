export let load;

async function init() {
	if (!import.meta.env.SSR) {
		({ load } = await import('rollup-plugin-wit-component'));
		console.log('wasm ready', { load });
	}
}

init();

// set up web worker messaging
onmessage = async (e) => {
	const { action, payload } = e.data;
	let result;
	// the modules created from wasmBytes and importables
	let mod;

	// loop until load function has been initialized
	while (!load) {
		await new Promise((resolve) => setTimeout(resolve, 100));
	}

	switch (action) {
		// takes the payload { arrayBuffer, importables} and loads them using load() from rollup-plugin-wit-component
		case 'load':
			try {
				mod = await load(payload.wasmBytes, payload.importables);
			} catch (e) {
				console.error('Error loading', e);
			}
			break;
		case 'render':
			// assert that mod is defined first, break early if not
			if (!mod) {
				console.error(
					'mod is not defined. Did you load the wasm module bytes and your importables?'
				);
				break;
			}
			try {
				rendered = await mod.render(payload.ctx);
			} catch (e) {
				console.error('Error rendering', e);
			}
			break;
		default:
			break;
	}
	postMessage({ action, payload: rendered });
};
