import tailwindcss from '@tailwindcss/vite'

import { sveltekit } from '@sveltejs/kit/vite'
import { defineConfig } from 'vite'

import fs from 'fs'
import path from 'path'

import { compile, escapeSvelte } from 'mdsvex'
import { codeToHtml } from 'shiki'

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
          }],

          // Custom extension to replace {DOC_VERSION} to current version in the readme.
          () => (tree: any) => {
            function replaceTextNodes(node: any) {
              if (typeof node.value === 'string') {
                node.value = node.value.replace(/({|&#123;)DOC_VERSION(}|&#125;)/g, DOC_VERSION)
              }

              console.log(node)

              if (node.children) {
                node.children.forEach(replaceTextNodes)
              }
            }

            replaceTextNodes(tree)
          }
        ],
        highlight: {
          highlighter: async (code: string, lang: string) => {
            return escapeSvelte(await codeToHtml(code, { lang, theme: 'kanagawa-wave' }));
          }
        }
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
