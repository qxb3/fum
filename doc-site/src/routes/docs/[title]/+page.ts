import type { PageLoad } from './$types'

import { error } from '@sveltejs/kit'

export const load: PageLoad = async ({ params }) => {
  const { title } = params

  const doc = DOCS
    .find(d =>
      d.path.split('/').slice(-2, -1)[0] === title
    )

  if (!doc)
    throw error(404, 'Documentation Not Found.')

  return {
    doc
  }
}
