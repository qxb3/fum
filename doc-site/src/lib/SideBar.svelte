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

<div class="w-96 border-r border-r-fg p-12">
  <ul class="list-none mt-4 text-lg space-y-2">
    {#each links as link}
      <li>
        <a
          href={link.href}>
          {link.text}
        </a>
      </li>
    {/each}
  </ul>
</div>
