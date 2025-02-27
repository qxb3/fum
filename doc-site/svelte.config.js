import { mdsvex } from 'mdsvex'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

import staticAdapter from '@sveltejs/adapter-static'

import rehypeSlug from 'rehype-slug'
import rehypeAutolinkHeadings from 'rehype-autolink-headings'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [
    vitePreprocess(),
    mdsvex({
      extensions: ['.md'],
        rehypePlugins: [
          rehypeSlug,
          [rehypeAutolinkHeadings, {
            behavior: 'wrap',
          }]
        ]
    })
  ],

  kit: {
    adapter: staticAdapter(),
    prerender: {
      entries: ['*']
    }
  },

  extensions: ['.svelte', '.md']
}

export default config
