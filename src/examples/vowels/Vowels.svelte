<script>
	import { onMount, tick } from 'svelte';
	import { WurboComponent } from 'wurbo';

	// Import wasm component bytes as a url
	import wasmURL from '../../../target/wasm32-wasi/debug/vowels.wasm?url';

	let name = 'demo:vowels';

	let value = "Let's count vowels using templates for Inputs and Outputs!";

	let data = {
		tag: 'all-content',
		val: {
			page: { title: value },
			input: { placeholder: "Input the word with vowels it's here" },
			output: { value: 'vowels' }
		}
	};

	// On update, send the new data to the component
	function handleUpdate() {
		data = {
			tag: 'phrase',
			val: value
		};
	}
</script>

<!-- A reactive Svelte code that allows the user here to edit the page.title to show Wurbo's reactivity to changing inputs -->
<p>Wurbo changes reactively to the input below:</p>
<input type="text" bind:value /><button on:click={handleUpdate}>Update</button>
{JSON.stringify(data)}
<WurboComponent {name} {wasmURL} {data} />
