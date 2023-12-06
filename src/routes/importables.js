// This  code must be stand alone (or bundled into a stand alone bundle)
// as it gets stringified and rolled up into the bundle of BobURL code that loads the wasm
export function prnt(string) {
	console.log(string);
}
/**
 * @param {string} ty - type of event to listen for
 * @param {function} handler - function to run when event is triggered
 */
export function addeventlistener({ selector, ty, value }) {
	const bc = new BroadcastChannel('listener_updates');
	let elem = document.querySelector(selector);
	document.querySelector(selector).addEventListener(ty, (e) => {
		// This code gets bundled into the main bundle, but there's no way for us
		// to reference `render` directly. But we can access the window object
		// which ensures we are using the same function. , we post a message to the parent window
		bc.postMessage(window.wurbo.render(e.target.value));
	});
}
