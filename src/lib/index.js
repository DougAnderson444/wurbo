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
			let data = event.data;
			// create an HTMLElement from the string, then extract the top most element id from the HTMLElement
			// Note: The top level element must have an id attribute! So we know what to replace
			function dom(data) {
				let id =
					new DOMParser().parseFromString(data || '', 'text/html')?.body?.firstElementChild?.id ||
					null;
				// if the id is not null, then we can update the html with the new string
				if (id) {
					let chosen = document.getElementById(id);
					if (chosen) {
						// @ts-ignore
						chosen.outerHTML = data;
						// if there are any event targets in this HTML, then we need to re-call mod.wurboOut.activate()
						// First, get all the id attributes from the data HTML
						let matching_ids = data.match(/id="[^"]*"/g)?.map((id) => '#' + id.slice(4, -1));

						try {
							mod?.wurboOut?.activate(matching_ids);
						} catch (e) {
							console.warn('No activate function for module: ', mod);
						}

						// In case Wurbo is being used with an aggregation module, we need to call aggregation.activates()
						try {
							mod?.aggregation?.activates(matching_ids);
						} catch (e) {
							console.info('Not an aggregation module. No aggregation.activates function found.');
						}

						return true;
					}
				}
				console.info(`No element found with id=${id} in ctx: \n ${data}`);
				return false;
			}

			if (dom(event.data)) return;

			let parsed = JSON.parse(event.data);

			// The other type of BroadcastChannel message is an event message, which wurbo
			// re-broadcasts via wurboOut.render(data). The components detect this
			// serialized message, deserde it into a Context and route it accordingly. Which means
			// the sender needs the Context type of the recipient and serde into that type. This is going
			// to work for both the JS runner and Rust Serde components. For example, if you want to put all
			// state changes in the #url hash, then listen on the BroadcastChannel in JS and change the hash.
			try {
				// Not all components will have listeners, so we wrap this in a try/catch to avoid ugly errors
				let rendered = mod.wurboOut.render(parsed);
				// in case this event refreshes the DOM, we use the new HTML to update the DOM
				dom(rendered);
			} catch (e) {
				console.warn('No listener found for event: ', event.data, e);
			}
		};
	}
}

/**
 * @param {string} namespace - The namespace of the Wurbo Class
 */
export function buildCodeString(namespace) {
	return `
      const bc = new BroadcastChannel('${namespace}');

      export function addeventlistener({ selector, ty }) {
        document.querySelector(selector).addEventListener(ty, (e) => {

          let val = e.target.value;

          // detect if form event
          if(e.target.closest('form')) {
            e.preventDefault();
          }

          let tag  = e.target.dataset.contextName || e.target.name;

          try {
            val = Object.assign({}, 
                    typeof JSON.parse(e.target.dataset.contextValue) === 'object' 
                    ? JSON.parse(e.target.dataset.contextValue) 
                    : {}, 
                    { value: e.target.value });
          } catch(e) {
            console.warn('Could not parse contextValue');
          }

          let ctx = { tag, val };

          let el = e.target.closest('[data-slot]');
          if(el) {
            ctx = { tag: el.dataset.slot, val: ctx };
            el = el.closest('[data-slot]');
          }

          // console.log({ctx});
          let rendered = window.${namespace}.render(ctx); 
          bc.postMessage(rendered);
        });
      }

      // Enables the guest components to emit a broadcast message to all peers on the same domain origin browsing context
      // Allows our wasm components to communicate with each other!
      export function emit(message) {
        bc.postMessage(message);
      }

      // Set hash of the current window to the given value
      export function setHash(hash) {
        window.location.hash = hash;
      }
`;
}
