<script>
	import { onMount } from 'svelte';

	// @ts-ignore
	// Import wasm wasm component bytes as a url
	import wasmURL from '../../crates/target/wasm32-wasi/release/hello.wasm?url';

	// get imports as a string
	import importables from './importables.js?raw';

	/**
	 * @type {string | null}
	 */
	let whatSayYou;
	/**
	 * @type {string}
	 */
	let code = 'Standby, generating your bundle...';

	onMount(async () => {
		const { load } = await import('rollup-plugin-wit-component');
		let wasmComponentBytesToESModule = await load();

		// get wasm bytes from url
		let wasmBytes = await fetch(wasmURL).then((res) => res.arrayBuffer());

		let importName = './importables.js';

		let imports = {
			map: {
				'component:cargo-comp': importName
			},
			files: [[importName, importables]]
		};

		let mod = await wasmComponentBytesToESModule({ wasmBytes, imprt: imports });

		// @ts-ignore
		console.log({ mod });
		whatSayYou = mod.render('World');
		console.log({ whatSayYou });
	});
</script>

<svelte:head>
	<title>Rollup Plugin WIT Demo</title>
</svelte:head>

{#if whatSayYou}
	{@html whatSayYou}
{/if}
