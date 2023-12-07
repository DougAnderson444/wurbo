import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

export const serverPath = '/wurbo';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter({
			pages: 'docs', // github pages
			assets: 'docs', // github pages
			fallback: 'index.html' // for static site prerendering
		}),
		paths: {
			base: serverPath // process.env.NODE_ENV === 'development' || process.argv.includes('dev') ? '' : serverPath
		},
		alias: {
			wurbo: './src/lib/index.js'
		}
	},
	preprocess: vitePreprocess()
};

export default config;
