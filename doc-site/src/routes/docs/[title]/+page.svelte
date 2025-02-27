<script lang="ts">
  import type { PageProps } from './$types'

  const { data }: PageProps = $props()
</script>

<main class="doc p-8">
  {@html data.doc.html}
</main>
