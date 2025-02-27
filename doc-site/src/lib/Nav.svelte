<script lang="ts">
  import { sideBarStore } from '$lib/stores'

  function toggleSideBar(e: MouseEvent) {
    e.stopPropagation()

    sideBarStore.update(state => !state)
  }
</script>

<nav class="px-4 py-2 bg-background text-fg border-b border-fg">
  <div class="grid grid-cols-3 text-2xl">
    <div class="flex items-center justify-start space-x-4">
      <button
        on:click={toggleSideBar}
        class="cursor-pointer transition-colors duration-300 hover:text-primary"
        aria-label="sidebar button">
        <i class="fa-solid fa-bars"></i>
      </button>
    </div>

    <a
      class="text-center hover:text-primary transition-colors duration-300"
      href="/">
      <h1 class="font-bold">fum documentation</h1>
    </a>

    <div class="list-none flex items-center justify-end space-x-4">
      <a
        class="text-background text-lg px-2 bg-fg hover:bg-primary transition-colors duration-300"
        href="https://github.com/qxb3/fum/releases/tag/v{DOC_VERSION}"
        target="_blank"
        aria-label="doc version">
        v{DOC_VERSION}
      </a>

      <a
        class="hover:text-primary transition-colors duration-300"
        href="https://discord.gg/UfXMeyZ6Zt"
        target="_blank"
        aria-label="discord invite">
        <i class="fa-brands fa-discord"></i>
      </a>

      <a
        class="hover:text-primary transition-colors duration-300"
        href="https://github.com/qxb3/fum"
        target="_blank"
        aria-label="github link">
        <i class="fa-brands fa-github"></i>
      </a>
    </div>
  </div>
</nav>
