import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [sveltekit()],
	server: { fs: { strict: false } },
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	resolve: {
		alias: {
			$examples: '/src/examples'
		}
	},
	// worker format
	worker: {
		format: 'es'
	},
	// build.minify false
	build: {
		minify: false
	}
});
