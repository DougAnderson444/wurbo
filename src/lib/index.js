// Make a class called Listener which constructs with a namespace field and has a method called listen
// The listen method takes a module and returns a namespace
export class Listener {
	constructor() {
		// add a random string to the end of the function name to avoid collisions
		let rand_string = Math.random().toString(36).substring(7);
		this.namespace = 'wurbo' + rand_string;
	}

	listen(mod) {
		// expose render function to blob URLs
		window[this.namespace] = { render: mod.wurboOut.render };

		// Set up a broadcast channel to listen for updates from the Blob URLs
		const bc = new BroadcastChannel(this.namespace);

		// Listen for messages from the Blob URLs
		bc.onmessage = (event) => {
			// create an HTMLElement from the string, then extract the top most element id from the HTMLElement
			let id =
				new DOMParser().parseFromString(event.data || '', 'text/html')?.body?.firstElementChild
					?.id || null;
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
}
