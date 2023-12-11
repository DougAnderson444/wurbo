// This code must be stand alone and 100% resolved into this single file
// as it gets stringified and rolled up into the bundle of BlobURL code that loads the wasm

// You could alternatively use the constant, but this would involve a bundle step to resolve all the code into a single file
// import { CHANNEL_NAME, GLOBAL_NAMESPACE} from 'wurbo';

export function prnt(string) {
	console.log(string);
}

/**
 * @param {{selector: string, ty: string, value: string}} param0 - The CSS selector we want to listen on, ty is the event type, value
 * @param {function} handler - function to run when event is triggered
 */
export function addeventlistener({ selector, ty, outputid }) {
	// The Broadcast channel name must be the same as the Wurbo
	// We could import the constant from Wurbo, but this would involve a bundle step to resolve all the code into a single file
	// Here we use it by string to show how it works
	// If you want to import and bundle it, go right ahead
	// const bc = new BroadcastChannel(CHANNEL_NAME);
	const bc = new BroadcastChannel('listener_updates');
	document.querySelector(selector).addEventListener(ty, (e) => {
		// This code gets bundled into the main bundle, but there's no way for us
		// to reference `render` directly. But we can access the window object
		// which ensures we are using the same function. We post a message to the parent window with the result
		console.log('target value', e.target.value);
		console.log({ outputid });
		// The API has changed. So now we need to chg the call too...
		let output_ctx = {
			name: e.target.value,
			// the output id
			id: outputid
		};
		let ctx = {
			page: { title: "Let's count vowels using templates for Inputs and Outputs!" },
			input: { placeholder: "Input the word with vowels it's here" },
			output: output_ctx
		};
		console.log({ ctx });
		let msg = window.wurbo.render(ctx);
		console.log('sending message', msg);
		bc.postMessage(msg);
		// alternatively, bundle with the exported constant:
		// bc.postMessage(window[GLOBAL_NAMESPACE].render(e.target.value));
	});
}
