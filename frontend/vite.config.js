import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    // Host and port can be set via FRONTEND_HOST / FRONTEND_PORT env vars
    host: process.env.FRONTEND_HOST || '0.0.0.0',
    port: parseInt(process.env.FRONTEND_PORT || '5173'),
  },
})
