import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import viteCompression from 'vite-plugin-compression';

// https://vite.dev/config/
export default defineConfig(({ command }) => {
	if (command === 'serve') {
		return {
			plugins: [viteCompression(), vue()],
			define: {
				__API_ENDPOINT__: JSON.stringify('http://localhost'),
				__API_ENDPOINT_STATIC__: JSON.stringify('http://static.localhost'),
			}
		}
	} else if (command === 'build') {
		return {
			plugins: [viteCompression(), vue()],
			define: {
				__API_ENDPOINT__: JSON.stringify('https://rosemary-artist.com'),
				__API_ENDPOINT_STATIC__: JSON.stringify('https//static.rosemary-artist.com'),
			}
		}
	} else {
		throw new Error('Invalid npm run script');
	}
})
