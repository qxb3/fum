<div class="h-full w-96 border-r border-r-fg">
  <div class="px-4 py-2 border-b border-fg text-2xl">
    <h1>Links</h1>
  </div>

  <ul class="h-full list-none mt-4 text-xl space-y-2 px-4">
    {#each DOCS as doc}
      <li>
        <a
          class="hover:text-primary"
          href={doc.url}>
          {doc.title}
        </a>
      </li>
    {/each}
  </ul>
</div>
