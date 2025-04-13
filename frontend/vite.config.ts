import tailwindcss from '@tailwindcss/vite';
import { svelteTesting } from '@testing-library/svelte/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

const is_production = process.env.NODE_ENV === 'production';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: !is_production
		? {
				watch: {
					usePolling: true,
					interval: 1000
				},
				proxy: {
					'/api': {
						target: 'http://backend:3000',
						changeOrigin: true,
						rewrite: (path) => path.replace(/^\/api/, ''),
					}
				}
			}
		: undefined,
	test: {
		workspace: [
			{
				extends: './vite.config.ts',
				plugins: [svelteTesting()],
				test: {
					name: 'client',
					environment: 'jsdom',
					clearMocks: true,
					include: ['src/**/*.svelte.{test,spec}.{js,ts}'],
					exclude: ['src/lib/server/**'],
					setupFiles: ['./vitest-setup-client.ts']
				}
			},
			{
				extends: './vite.config.ts',
				test: {
					name: 'server',
					environment: 'node',
					include: ['src/**/*.{test,spec}.{js,ts}'],
					exclude: ['src/**/*.svelte.{test,spec}.{js,ts}']
				}
			}
		]
	}
});
