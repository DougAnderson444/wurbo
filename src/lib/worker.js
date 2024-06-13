import { load } from 'rollup-plugin-wit-component';

// the modules created from wasmBytes and importables
let mod;

// set up web worker messaging
addEventListener('message', async (e) => {
	const { action, payload, messageId } = e.data;
	let rendered;

	switch (action) {
		// takes the payload { arrayBuffer, importables} and loads them using load() from rollup-plugin-wit-component
		case 'load':
			try {
				mod = await load(payload.arrayBuffer, payload.importables);
				// if payload.templates, call mod.customize(templates)
				if (payload?.templates) {
					mod.wurboOut.customize(payload.templates);
				}
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
				console.warn('Cannot render: ', e);
				break;
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
				break;
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
				break;
			}
			break;
		default:
			break;
	}
	postMessage({ action, payload: rendered, messageId });
});
