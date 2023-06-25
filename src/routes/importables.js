export const cargoCompImports = {
	prnt(string) {
		console.log('from importables.js: ', string);
	},
	/**
	 * @param {string} ty - type of event to listen for
	 * @param {function} handler - function to run when event is triggered
	 */
	addeventlistener(selector, ty, value) {
		const bc = new BroadcastChannel('listener_updates');
		let elem = document.querySelector(selector);
		document.querySelector(selector).addEventListener(ty, (e) => {
			bc.postMessage(window.render(value));
		});
	}
};
