import { DEV, BROWSER } from 'esm-env';

// The load function
let load;
// the modules created from wasmBytes and importables
let mod;

async function init() {
	if (BROWSER) {
		({ load } = await import('rollup-plugin-wit-component'));
		console.log('wasm ready', { load });
	}
}

init();

// set up web worker messaging
onmessage = async (e) => {
	const { action, payload } = e.data;
	let rendered;

	// loop until load function has been initialized
	while (!load) {
		await new Promise((resolve) => setTimeout(resolve, 100));
	}

	switch (action) {
		// takes the payload { arrayBuffer, importables} and loads them using load() from rollup-plugin-wit-component
		case 'load':
			try {
				console.time('load');
				mod = await load(payload.arrayBuffer, payload.importables);
				console.timeEnd('load');
				console.log('modules loaded.', mod);
			} catch (e) {
				console.error('Error loading', e);
			}
			break;
		case 'render':
			let i = 0;
			while (!(mod && mod?.wurboOut && mod?.wurboOut?.render)) {
				if (i > 10) {
					console.warn('Condition not met');
					break;
				}
				await new Promise((resolve) => setTimeout(resolve, 100));
				i++;
			}

			try {
				rendered = await mod.wurboOut.render(payload);
			} catch (e) {
				console.error('Error rendering', e);
			}
			break;
		// mod.wurboOut.activate(selectors ids)
		// This function is called when the HTML is updated and we need to re-activate the event listeners
		case 'activate':
			let j = 0;
			while (!(mod && mod?.wurboOut && mod?.wurboOut?.activate)) {
				if (j > 10) {
					console.warn('Condition not met');
					break;
				}
				await new Promise((resolve) => setTimeout(resolve, 100));
				j++;
			}

			try {
				mod.wurboOut.activate(payload);
			} catch (e) {
				console.warn('No activate function for module.');
			}
			break;
		// mod?.aggregation?.activates(matching_ids)
		// This function is called when the HTML is updated and we need to re-activate the event listeners
		case 'aggregation':
			let k = 0;
			while (!(mod && mod?.aggregation && mod?.aggregation?.activates)) {
				if (k > 10) {
					break;
				}
				await new Promise((resolve) => setTimeout(resolve, 100));
				k++;
			}

			try {
				mod?.aggregation?.activates(payload);
			} catch (e) {
				console.warn('No aggregation.activates function for module: ', mod);
			}
			break;
		default:
			break;
	}
	postMessage({ action, payload: rendered });
};

// Function to asset that mod has loaded
async function assertVal(cond) {
	let i = 0;
	while (!cond) {
		if (i > 10) {
			console.warn('Condition not met');
			break;
		}
		await new Promise((resolve) => setTimeout(resolve, 100));
		i++;
	}
}