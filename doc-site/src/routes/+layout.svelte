<script lang="ts">
  import '../app.css'

  import Nav from '$lib/Nav.svelte'
  import SideBar from '$lib/SideBar.svelte'

  const { children } = $props()
</script>


<SideBar />

<div class="h-full">
  <Nav />

  {@render children()}
</div>
