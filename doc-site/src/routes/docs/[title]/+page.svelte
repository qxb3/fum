<script lang="ts">
  import type { PageProps } from './$types'

  const { data }: PageProps = $props()
</script>

<main class="doc p-8">
  <div class="overflow-x-hidden">
    {@html data.doc.html}
  </div>

  <hr>

  <div class="flex justify-between items-center">
    {#if data.doc.prev}
      <a
        class="self-start font-bold text-fg! text-4xl cursor-pointer hover:text-background duration-300 transition-colors"
        href={data.doc.prev.url}>
        <span>
          <i class="fa-solid fa-chevron-left"></i>
        </span>
        <span>{data.doc.prev.title}</span>
      </a>
    {:else}
      <div></div>
    {/if}

    {#if data.doc.next}
      <a
        class="font-bold text-fg! text-4xl cursor-pointer hover:text-background duration-300 transition-colors"
        href={data.doc.next.url}>
        <span>{data.doc.next.title}</span>
        <span>
          <i class="fa-solid fa-chevron-right"></i>
        </span>
      </a>
    {/if}
  </div>
</main>
