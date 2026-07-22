import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
// In production the site is served from the GitHub Pages project subpath
// (https://<user>.github.io/satisfactory-calculator/), so the build needs an
// absolute base. Local dev stays at the root for convenience.
export default defineConfig(({ command }) => ({
  base: command === 'build' ? '/satisfactory-calculator/' : '/',
  plugins: [svelte()],
}))