import tailwindcss from '@tailwindcss/vite'

import { sveltekit } from '@sveltejs/kit/vite'
import { defineConfig } from 'vite'

import fs from 'fs'
import path from 'path'

import { compile } from 'mdsvex'

import rehypeSlug from 'rehype-slug'
import rehypeAutolinkHeadings from 'rehype-autolink-headings'

const DOC_VERSION = fs.readFileSync('DOC_VERSION', 'utf-8').trim()
const DOCS = await getDocs('docs-content')

export default defineConfig({
  plugins: [
    sveltekit(),
    tailwindcss()
  ],
  define: {
    DOC_VERSION: JSON.stringify(DOC_VERSION),
    DOCS: JSON.stringify(DOCS)
  }
})

async function getDocs(docsPath: string) {
  const docs = await Promise.all(fs
    .readdirSync(docsPath)
    .map(async _docPath => {
      const docPath = path.join(__dirname, docsPath, _docPath, 'doc.md')
      const raw = fs.readFileSync(docPath, 'utf-8')
      const compiledContent = await compile(raw, {
        rehypePlugins: [
          rehypeSlug,
          [rehypeAutolinkHeadings, {
            behavior: 'wrap',
          }]
        ]
      })

      return {
        url: `/docs/${_docPath}`,
        path: docPath,
        raw,
        title: compiledContent.data.fm?.title,
        html: compiledContent.code
      }
    }))

  return docs
}
