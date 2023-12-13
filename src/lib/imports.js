import { LISTENER_UPDATES } from './index.js';

/**
 * @param {string} ty - type of event to listen for
 * @param {function} handler - function to run when event is triggered
 */
export function addeventlistener({ selector, ty, value }) {
	const bc = new BroadcastChannel(LISTENER_UPDATES);
	let elem = document.querySelector(selector);
	document.querySelector(selector).addEventListener(ty, (e) => {
		// This code gets bundled into the main bundle, but there's no way for us
		// to reference `render` directly. But we can access the window object
		// which ensures we are using the same function. , we post a message to the parent window
		let output_ctx = {
			tag: 'output',
			val: {
				value: e.target.value,
				template
			}
		};
		bc.postMessage(window[GLOBAL_NAMESPACE].render(output_ctx));
	});
}
