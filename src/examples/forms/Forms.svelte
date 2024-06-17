<script>
	import { onMount, tick } from 'svelte';
	import { WurboComponent } from 'wurbo';

	import index from './templates/index.html?raw';
	import input from './templates/input.html?raw';
	import output from './templates/output.html?raw';

	// Import wasm component bytes as a url
	import wasmURL from '../../../target/wasm32-wasi/debug/forms.wasm?url';

	// We can pass in customized templates to the component
	let templates = [
		['index.html', index],
		['input.html', input],
		['output.html', output]
	];

	let data = {
		page: { title: "Let's process the contents of a form." },
		input: { placeholder: 'Enter a Username here' }
	};

	const eventHandler = (payload) => {
		console.log('Form eventHandler', payload);
	};
</script>

<WurboComponent {wasmURL} {data} {templates} {eventHandler} />
