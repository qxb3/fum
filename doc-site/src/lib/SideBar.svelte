<script lang="ts">
  import { sideBarStore } from '$lib/stores'

  function closeSideBar() {
    sideBarStore.set(false)
  }
</script>

<div
  class="fixed top-0 w-96 h-screen border-r border-r-fg z-50 bg-background transition-all duration-300"
  class:left-0={$sideBarStore}
  class:-left-96={!$sideBarStore}>
  <div class="relative">
    <button
      class="absolute top-0 right-0 m-4 cursor-pointer text-md"
      aria-label="close sidebar"
      on:click={closeSideBar}>
      <i class="fa-solid fa-xmark"></i>
    </button>
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
