import tailwindcss from '@tailwindcss/vite'

import { sveltekit } from '@sveltejs/kit/vite'
import { defineConfig } from 'vite'

import fs from 'node:fs'
import path from 'node:path'

import { compile, escapeSvelte } from 'mdsvex'
import { codeToHtml, type ShikiTransformer } from 'shiki'

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

interface addCodeblockCopyButtonOptions {
  /** `3000` by default */
  ms?: number
}
function addCodeblockCopyButton ({
  ms = 3000
}: addCodeblockCopyButtonOptions = {}) {
  return {
    name: 'shiki-transformer-codeblock-copy-button',
    pre(node) {
      node.children.push({
        type: 'element',
        tagName: 'button',
        properties: {
          className: [ 'copy' ],
          "data-code": this.source,
          onClick: `
            navigator.clipboard.writeText(this.dataset.code);
            this.setAttribute('disabled', true);
            this.classList.add('copied');
            setTimeout(() => {
              this.classList.remove('copied');
              this.removeAttribute('disabled');
            }, ${ms})`
        },
        children: [
          {
            type: 'element',
            tagName: 'span',
            properties: { className: [ 'ready' ] },
            children: []
          },
          {
            type: 'element',
            tagName: 'span',
            properties: { className: [ 'success' ] },
            children: []
          }
        ]
      });
    }
  } satisfies ShikiTransformer;
}

async function getDocs(docsPath: string) {
  const docs = await Promise.all(fs
    .readdirSync(docsPath)
    .sort((a, b) => a.localeCompare(b, undefined, { numeric: true }))
    .map(async docPath => {
      const mdPath = path.join(__dirname, docsPath, docPath, 'doc.md')
      const content = fs.readFileSync(mdPath, 'utf-8')
      const compiledContent = await compile(content, {
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

              if (node.children) {
                node.children.forEach(replaceTextNodes)
              }
            }

            replaceTextNodes(tree)
          }
        ],
        highlight: {
          highlighter: async (code: string, lang: string) => {
            return escapeSvelte(
              await codeToHtml(
                code,
                {
                  lang,
                  theme: 'kanagawa-wave',
                  transformers: [addCodeblockCopyButton()],
                  tabindex: -1
                }
              )
            );
          }
        }
      })

      const prev = compiledContent.data.fm!.prev?.split(':')
      const next = compiledContent.data.fm!.next?.split(':')

      return {
        url: `/docs/${docPath.slice(3)}`,
        raw: content,
        title: compiledContent.data.fm!.title,
        prev: prev ? { url: prev[0], title: prev[1] } : undefined,
        next: next ? { url: next[0], title: next[1] } : undefined,
        html: compiledContent.code
      }
    }))

  return docs
}
