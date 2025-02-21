<script lang="ts">
  import '../app.css'

  import Nav from '$lib/Nav.svelte'

  let { children } = $props()
</script>

<Nav />

{@render children()}
