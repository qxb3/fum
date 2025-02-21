<script lang="ts">
  function toggleSideBar() {
  }
</script>

<nav class="px-4 py-2 bg-background text-fg border-b border-fg">
  <div class="grid grid-cols-3 text-xl">
    <div class="flex items-center justify-start space-x-4">
      <button
        on:click={toggleSideBar}
        class="cursor-pointer"
        aria-label="sidebar button">
        <i class="fa-solid fa-bars"></i>
      </button>
    </div>

    <a class="text-center" href="/">
      <h1 class="font-bold">fum documentation</h1>
    </a>

    <li class="list-none flex items-center justify-end space-x-4">
      <ul>
        <a
          class="text-background text-sm px-2 py-1 bg-fg"
          href="https://github.com/qxb3/fum/releases/tag/v{DOC_VERSION}"
          target="_blank"
          aria-label="doc version">
          v{DOC_VERSION}
        </a>
      </ul>

      <ul>
        <a
          href="https://github.com/qxb3/fum"
          target="_blank"
          aria-label="github link">
          <i class="fa-brands fa-github"></i>
        </a>
      </ul>
    </li>
  </div>
</nav>
