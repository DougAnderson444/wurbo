<script>
	import { onMount, tick } from 'svelte';
	import { WurboComponent } from 'wurbo';

	// Import wasm component bytes as a url
	import wasmURL from '../../../target/wasm32-wasi/debug/vowels.wasm?url';

	let name = 'demo:vowels';

	let data = {
		page: { title: "Let's count vowels using templates for Inputs and Outputs!" },
		input: { placeholder: "Input the word with vowels it's here" },
		output: { value: 'vowels' }
	};
</script>

<WurboComponent {name} {wasmURL} {data} />
