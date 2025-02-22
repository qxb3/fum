<script lang="ts">
  import '../app.css'

  import Nav from '$lib/Nav.svelte'
  import SideBar from '$lib/SideBar.svelte'

  let { children } = $props()
</script>

<div class="flex min-h-full">
  <SideBar />

  <div class="w-full min-h-full">
    <Nav />

    {@render children()}
  </div>
</div>
