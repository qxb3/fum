<script lang="ts">
  const links = [
    {
      href: '/foo',
      text: 'Foo'
    },
    {
      href: '/bar',
      text: 'Bar'
    },
    {
      href: '/xd',
      text: 'xd'
    },
  ]
</script>

<div class="w-96 border-r border-r-fg">
  <div class="px-4 py-2 border-b border-fg text-xl">
    <h1>Links</h1>
  </div>

  <ul class="list-none mt-4 text-lg space-y-2 px-4">
    {#each links as link}
      <li>
        <a
          class="hover:text-primary"
          href={link.href}>
          {link.text}
        </a>
      </li>
    {/each}
  </ul>
</div>
