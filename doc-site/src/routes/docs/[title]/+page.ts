import type { PageLoad, EntryGenerator } from './$types'

import { error } from '@sveltejs/kit'

export const load: PageLoad = async ({ params }) => {
  const { title } = params

  const doc = DOCS
    .find(d =>
      d.title.toLowerCase().replaceAll(' ', '_') === title
    )

  if (!doc)
    throw error(404, 'Documentation Not Found.')

  return {
    doc
  }
}

export const entries: EntryGenerator = () => {
  return DOCS.map(d => ({ title: d.title.toLowerCase().replaceAll(' ', '_') }))
}

export const prerender = true
