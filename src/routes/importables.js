export const cargoCompImports = {
	prnt(string) {
		console.log(string);
	},
	/**
	 * @param {string} ty - type of event to listen for
	 * @param {function} handler - function to run when event is triggered
	 */
	addeventlistener({ selector, ty, value }) {
		const bc = new BroadcastChannel('listener_updates');
		let elem = document.querySelector(selector);
		document.querySelector(selector).addEventListener(ty, (e) => {
			// Because this cod eis in a blob:URL, we can't access the window object
			// directly. Instead, we post a message to the parent window
			bc.postMessage(window.render(e.target.value));
		});
	}
};
