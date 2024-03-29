import { build } from 'vite';
import path from 'path';
import { fileURLToPath } from 'url';
import { minify } from 'terser';
import terser from '@rollup/plugin-terser';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

async function buildWorker() {
	let name = 'worker';
	let outDir = 'src/lib/bundled';
	await build({
		configFile: false,
		build: {
			outDir,
			lib: {
				entry: path.resolve(__dirname, `./src/lib/${name}.js`),
				fileName: name,
				name
			},
			emptyOutDir: false,
			minify: true,
			sourcemap: false,
			rollupOptions: {
				output: [
					{
						sourcemap: false,
						format: 'es',
						dir: outDir,
						manualChunks: false,
						inlineDynamicImports: true,
						name: 'app',
						compact: true,
						plugins: [terser()]
					}
				],
				plugins: [terser()]
			}
		}
	});
	console.log('Worker built.');
}

(async () => {
	await buildWorker();
})();
