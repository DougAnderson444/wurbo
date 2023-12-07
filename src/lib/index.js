// The main library file
// exports all the functions that are available to the user
import { LISTENER_UPDATES } from './constants.js';

function listen(mod) {
	// expose render function to blob URLs
	window.wurbo = { render: mod.render };

	// Set up a broadcast channel to listen for updates from the Blob URLs
	const bc = new BroadcastChannel(LISTENER_UPDATES);

	// Listen for messages from the Blob URLs
	bc.onmessage = (event) => {
		// create an HTMLElement from the string, then extract the top most element id from the HTMLElement
		let id =
			new DOMParser().parseFromString(event.data || '', 'text/html')?.body?.firstElementChild?.id ||
			null;
		// if the id is not null, then we can update the html with the new string
		if (id) {
			// @ts-ignore
			let chosen = document.getElementById(id);
			if (chosen) {
				// @ts-ignore
				chosen.outerHTML = event.data;
				return;
			}
		}
		console.warn('No element found with id: ', id);
	};
}

export { listen, LISTENER_UPDATES };
