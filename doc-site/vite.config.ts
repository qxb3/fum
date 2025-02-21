import tailwindcss from '@tailwindcss/vite'

import { sveltekit } from '@sveltejs/kit/vite'
import { defineConfig } from 'vite'
import { readFileSync } from 'fs'

const docVersion = readFileSync('DOC_VERSION', 'utf-8').trim()

export default defineConfig({
  plugins: [
    sveltekit(),
    tailwindcss()
  ],
  define: {
    DOC_VERSION: JSON.stringify(docVersion)
  }
})
