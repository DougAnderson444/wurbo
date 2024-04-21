import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { purgeCss } from 'vite-plugin-tailwind-purgecss';

export default defineConfig({
	plugins: [sveltekit(), purgeCss()],
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
