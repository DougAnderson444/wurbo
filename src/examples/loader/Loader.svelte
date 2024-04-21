<script>
	import { WurboLoader } from 'wurbo';

	let preload =
		'https://github.com/PeerPiper/peerpiper/releases/download/prerelease/peerpiper_wallet_aggregate.wasm';
</script>

<WurboLoader {preload} />

<details class="flex flex-col pt-4 cursor-pointer">
	<summary>Details</summary>
	<h2>What kinds of files can I load?</h2>
	<p>Only <span><code>.wasm</code></span> files are supported.</p>
	<h2>What is a <span><code>.wasm</code></span> file?</h2>
	<p>
		WebAssembly is a binary instruction format for a stack-based virtual machine. Wasm is designed
		as a portable target for compilation of high-level languages like C/C++/Rust, enabling
		deployment on the web for client and server applications.
	</p>
	<h2>Does the wasm need to implement <code>Wurbo</code>?</h2>
	<p>
		Yes, the wasm file must export a function named <code>wurbo-in</code> and import a function
		named <code>wurbo-out</code>.
	</p>
</details>
