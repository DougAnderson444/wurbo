// This code must be stand alone and 100% resolved into this single file
// as it gets stringified and rolled up into the bundle of BlobURL code that loads the wasm

// You could alternatively use the constant, but this would involve a bundle step to resolve all the code into a single file
// import { LISTENER_UPDATES } from 'wurbo';

export function prnt(string) {
	console.log(string);
}

/**
 * @param {string} ty - type of event to listen for
 * @param {function} handler - function to run when event is triggered
 */
export function addeventlistener({ selector, ty, value }) {
	// The Broadcast channel name must be the same as the Wurbo
	// We could import the constant from Wurbo, but this would involve a bundle step to resolve all the code into a single file
	// Here we use it by string to show how it works
	// If you want to import and bundle it, go right ahead
	const bc = new BroadcastChannel('listener_updates');
	let elem = document.querySelector(selector);
	document.querySelector(selector).addEventListener(ty, (e) => {
		// This code gets bundled into the main bundle, but there's no way for us
		// to reference `render` directly. But we can access the window object
		// which ensures we are using the same function. , we post a message to the parent window
		bc.postMessage(window.wurbo.render(e.target.value));
	});
}
