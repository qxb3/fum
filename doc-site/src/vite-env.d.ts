/// <reference types="vite/client" />

declare const DOC_VERSION; string
declare const DOCS: {
  url: string
  path: string
  raw: string
  title: string
  html: string
  prev?: {
    url: string
    title: string
  }
  next?: {
    url: string
    title: string
  }
}[]
