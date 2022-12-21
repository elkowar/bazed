<!--
    LinesView contains lines of text
-->
<script
  lang="ts"
  context="module"
>
  import type { Writable } from "svelte/store"
  import { writable } from "svelte/store"

  import type { Vector2 } from "./LinearAlgebra"
  import { arrayUpdate, stringSplice } from "./Common"

  export type Line = string

  export const lines: Writable<string[]> = writable([]) // contains all cached lines

  export const isAlpha = (code: number): boolean =>
    (code > 47 && code < 58) || (code > 64 && code < 91) || (code > 96 && code < 123)

  // insert
  export const atInsert = (text: string, [x, y]: Vector2): void =>
    lines.update((lines) => {
      arrayUpdate(lines, y, (line) => stringSplice(line, x, text))
      return lines
    })

  export const atRemove = ([x, y]: Vector2): void =>
    lines.update((lines) => {
      arrayUpdate(lines, y, (line) => line.substring(0, x) + line.substring(x + 1))
      return lines
    })

  export const insertLine = (i: number, line: string): void =>
    lines.update((lines) => {
      lines.splice(i, 0, line)
      return lines
    })

  export const longestLine = (text: string[]): Line =>
    text.reduce((a, b) => (a.length < b.length ? b : a))
</script>

<script lang="ts">
  import type { Theme } from "./Theme"
  export let theme: Theme
  export let line_height: number

  export let height: number
  export let width: number

  $: height = $lines.length * line_height
  $: width = $lines ? longestLine($lines).length : 1
</script>

<div class="lines-container">
  {#each $lines as line, i}
    <div
      class="line-container"
      style:top="{i * line_height}px"
      style:height="{line_height}px"
    >
      <span
        class="line-view"
        style:font-family={theme.font.family}
        style:font-size={theme.font.size}
        style:color={theme.text_color}
        style:height="{line_height}px"
        style:line-height="{line_height}px"
      >
        {line}
      </span>
    </div>
  {/each}
</div>

<style>
  .lines-container {
    position: absolute;
  }

  .line-container {
    position: absolute;
    width: 100%;
    cursor: text;
  }

  .line-view {
    white-space: pre;
  }
</style>
