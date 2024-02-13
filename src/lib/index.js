// Make a class called Listener which constructs with a namespace field and has a method called listen
// The listen method takes a module and returns a namespace
export class Wurbo {
	// takes the given arrayBuffer and importables string and creates the WebWorker,
	// using transferable objects to avoid copying the arrayBuffer
	constructor({ arrayBuffer, importables }) {
		// create a new WebWorker with the current file path
		const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });

		// post a message to the worker with the action 'load' and the payload { arrayBuffer, importables }
		worker.postMessage({ action: 'load', payload: { arrayBuffer, importables } }, [arrayBuffer]);

		// helper function to post message to worker and process responses
		this.post = (action, payload) => {
			// create a message id to track each response, in case there are multiple requests with the same action
			// this is useful for the render method, which can be called multiple times
			// and we want to make sure we are getting the correct response for each request
			const messageId = Math.random().toString(36).slice(2);
			// console.time(action + ' - ' + messageId);
			//
			// post a message to the worker with the action and payload
			// and the message id to track the response
			// the worker will respond with the same message id, so we can match the response to the request
			// and resolve the promise with the payload
			return new Promise((resolve) => {
				const handler = (e) => {
					if (e.data.action === action && e.data.messageId === messageId) {
						resolve(e.data.payload);
						//console.timeEnd(action + ' - ' + messageId);
						worker.removeEventListener('message', handler);
					}
				};
				worker.onmessage = handler;
				worker.postMessage({ action, payload, messageId });
			});
		};

		// Message event listener for 'other' messages emitted from the worker
		worker.addEventListener('message', (e) => {
			let { action, payload } = e.data;
			switch (action) {
				case 'addeventlistener':
					this.addeventlistener(payload);
					break;
				case 'setHash':
					window.location.hash = payload;
					break;
				case 'emit':
					this.dom(payload);
					break;
			}
		});
	}

	async render(ctx) {
		return this.post('render', ctx);
	}

	// activate the css selectors
	async activate(selectors = null) {
		return this.post('activate', selectors);
	}

	// aggregation.activates (plural) CSS selectors
	async aggregation(selectors = null) {
		return this.post('aggregation', selectors);
	}

	// create an HTMLElement from the string, then extract the top most element id from the HTMLElement
	// Note: The top level element must have an id attribute! So we know what to replace
	async dom(data) {
		const updateHTML = async (data) => {
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
						await this.post('activate', matching_ids);
					} catch (e) {
						console.warn('No activate function for module');
					}

					// In case Wurbo is being used with an aggregation module, we need to call aggregation.activates()
					try {
						await this.post('aggregation', matching_ids);
					} catch (e) {
						console.info('Not an aggregation module. No aggregation.activates function found.');
					}

					return true;
				}
			}
			console.info(`No element found with id in ctx`);
			return false;
		};

		// The first type of render request is a String of HTML:
		if (await updateHTML(data)) return;

		// The other type of DOM access request is from an event message, stringified JSON object, which needs
		// to be parsed into a JSON object first before it is rendered.
		// This is for event listeners to handle the contents of the stringified JSON object, and then potentially update the DOM
		try {
			let parsed = JSON.parse(data);
			// Not all components will have listeners, so we wrap this in a try/catch to avoid ugly errors
			// let rendered = mod.wurboOut.render(parsed);
			let rendered = await this.post('render', parsed);
			// in case this event refreshes the DOM, we use the new HTML to update the DOM
			await updateHTML(rendered);
		} catch (e) {
			console.warn('No listener found for event: ', event.data, e);
			// If the data is not a stringified JSON object, then try to pass it as a string (base64)
			try {
				let rendered = await this.post('render', data);
				await updateHTML(rendered);
			} catch (e) {
				console.trace('Cannot render object or string for event', event.data, e);
			}
		}
	}

	async addeventlistener({ selector, ty }) {
		document.querySelector(selector).addEventListener(ty, async (e) => {
			let val = e.target.value;

			// detect if form event
			if (e.target.closest('form')) {
				e.preventDefault();
			}

			let tag = e.target.dataset.contextName || e.target.name;

			try {
				val = Object.assign(
					{},
					typeof JSON.parse(e.target.dataset.contextValue) === 'object'
						? JSON.parse(e.target.dataset.contextValue)
						: {},
					{ value: e.target.value }
				);
			} catch (e) {
				// It's ok to be missing a contextValue. Just means only e.target.value will be sent.
				// console.warn('Could not parse contextValue', e);
			}

			let ctx = { tag, val };

			let el = e.target.closest('[data-slot]');
			if (el) {
				ctx = { tag: el.dataset.slot, val: ctx };
				el = el.closest('[data-slot]');
			}

			let rendered = await this.post('render', ctx);
			this.dom(rendered);
		});
	}
}

/**
 * These functions are called from within the WebWorker, so this needs to post this request back to the main thread
 * so that the DOM events can be manipulated.
 */
export const wurboIn = `
      export function addeventlistener({ selector, ty }) {
        postMessage({ action: 'addeventlistener', payload: { selector, ty } });
      }

      export function emit(message) {
        console.log('worker emit');
        postMessage({ action: 'emit', payload: message });
      }

      // Set hash of the current window to the given value
      export function setHash(hash) {
        // window.location.hash = hash;
        postMessage({ action: 'setHash', payload: hash });
      }
`;
