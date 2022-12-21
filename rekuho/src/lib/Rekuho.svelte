<script
  lang="ts"
  context="module"
>
</script>

<script lang="ts">
  import { example as exampleTheme } from "./Theme"
  import { websocket } from "./Rpc"

  import Portion from "./Portion.svelte"

  export let width: number = 500
  export let height: number = 500

  let theme = exampleTheme
  websocket()
</script>

<div
  class="rekuho"
  style:width
>
  <Portion
    bind:theme
    {width}
    {height}
  />
</div>
