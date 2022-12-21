<!--
    Portion, may be simply the Editor
    This window contains the visible and editable text.
-->
<script lang="ts">
  import type { Theme } from "./Theme"
  import LinesView, { lines, isAlpha, atInsert, atRemove, insertLine } from "./LinesView.svelte"
  import type { Position } from "./Cursors.svelte"
  import CursorsLayer, { cursors, cursorUpdate, cursorMove } from "./Cursors.svelte"

  export let theme: Theme
  const gutter_width = 50 // maybe should be part of theme, minimum value?

  export let height: number
  export let width: number

  let view: Element
  let input: HTMLTextAreaElement
  let container: Element

  // TODO: get proper input from backend
  lines.update((_) => [
    ...new Array(10).fill(""),
    ..."funky\nbanana\nt0wn".split("\n"),
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    ...new Array(20).fill("a"),
  ])

  // let portion_start_line: number = 0

  const getCharacterWidth = (font: string): number | null => {
    const canvas = new OffscreenCanvas(0, 0)
    const context = canvas.getContext("2d") as OffscreenCanvasRenderingContext2D | null
    if (context) {
      context.font = font
    }
    // FIXME: elkowar mentioned: "with a font of **ZERO WIDTH X-CHARACTERS**" this breaks.
    // It does. ture. (typo intended)
    return context?.measureText("X").width || null
  }

  // TODO: separate into linear_algebra.ts
  $: view_rect = container && container.getBoundingClientRect()
  const pxToPortionPosition = ([x, y]: Position): Position => {
    const div = (x: number, y: number): number => Math.floor(x / y)
    const column = div(x - view_rect.x, column_width)
    const line = div(y - view_rect.y, line_height)
    return [column, line]
  }

  const font = theme.font
  const line_height: number = 18
  const column_width: number = getCharacterWidth(`${font.weight} ${font.size} ${font.family}`) || 1

  // TODO: implement selections
  let selection: Position | null = null

  ////////////////////////////////////////////////////////////////////////////////

  const mousedown = (ev: MouseEvent) => {
    const current = pxToPortionPosition([ev.pageX, ev.pageY])
    selection = current
    cursorUpdate(0, (_) => current)
    input.focus()
  }

  const mouseup = (ev: MouseEvent) => {
    if (selection) {
      const begin = selection
      selection = null
      const end = pxToPortionPosition([ev.pageX, ev.pageY])
    }
  }

  const mousemove = (ev: MouseEvent) => {
    if (selection) {
      cursorUpdate(0, (_) => pxToPortionPosition([ev.pageX, ev.pageY]))
    }
  }

  // TODO: handle drag events
  // const dragstart = (ev: DragEvent) => {}
  // const drag = (ev : DragEvent) => {}

  const keydown = (ev: KeyboardEvent) => {
    // TODO: handle input from keydown events
    if (ev.key.length === 1) {
      atInsert(ev.key, $cursors[0].pos)
      cursorMove(0, [1, 0])
    }

    switch (ev.key) {
      case "Enter": {
        insertLine($cursors[0].pos[1] + 1, "")
        cursorMove(0, [0, 1])
        break
      }
      case "Backspace": {
        atRemove($cursors[0].pos)
        cursorMove(0, [-1, 0])
        break
      }
    }
  }

  const gutter_mousedown = (line: number, ev: MouseEvent) => {
    cursorUpdate(0, (_) => [null, line])
  }

  ////////////////////////////////////////////////////////////////////////////////

  let line_view_height: number
  let line_view_width: number

  $: text_view_width = width - gutter_width
  $: text_to_visible_ratio = (line_view_width * column_width - theme.text_offset) / width
  $: vertical_scroller_width = text_view_width / text_to_visible_ratio
</script>

<div
  class="view"
  bind:this={view}
  style:width="{width}px"
  style:height="{height}px"
>
  <!--<GutterColumn line_height={line_height} />-->
  <!-- TODO: refactor into `Gutter.svelte` -->
  <div
    class="gutter"
    style:background={theme.gutter.background}
    style:width="{gutter_width}px"
    style:height="{height}px"
  >
    {#each $lines as _, i}
      <div
        class="gutter-cell"
        on:mousedown|preventDefault={(e) => {
          gutter_mousedown(i, e)
        }}
        style:font-size={theme.font.size}
        style:height="{line_height}px"
        style:top="{i * line_height}px"
      >
        {i + 1}
      </div>
    {/each}
  </div>

  <!-- don't know if this thing ought to exist at all -->
  <div
    class="text-offset-background"
    style:position="absolute"
    style:height="{height}px"
    style:top="0"
    style:background={theme.editor_background}
    style:width="{theme.text_offset}px"
    style:left="{gutter_width}px"
  />

  <div
    bind:this={container}
    class="container"
    on:mousedown|preventDefault={mousedown}
    on:mousemove={mousemove}
    on:mouseup={mouseup}
    style:background={theme.editor_background}
    style:left="{gutter_width + theme.text_offset}px"
  >
    <textarea
      bind:this={input}
      tabindex="-1"
      wrap="off"
      on:keydown|preventDefault={keydown}
      style:user-select="text"
      style:position="absolute"
      style:width="{column_width}px"
    />
    <LinesView
      bind:theme
      bind:height={line_view_height}
      bind:width={line_view_width}
      {line_height}
    />
    <CursorsLayer
      bind:theme
      {column_width}
      {line_height}
    />
  </div>

  <!-- TODO: Implement scrollbars -->
  <!-- <Scrollbar /> -->
  <!-- <Scrollbar /> -->
  <div
    class="scrollbar vertical"
    style:height="{theme.scrollbar_width}px"
    style:width="{width - gutter_width}px"
    style:left="{gutter_width}px"
    style:top="{height - theme.scrollbar_width}px"
  >
    <div
      class="scroller"
      on:mousedown|preventDefault={(e) => {}}
      style:height="{theme.scrollbar_width}px"
      style:width="{vertical_scroller_width}px"
      style:background="#FFFFFF"
    />
  </div>
</div>

<style>
  .view {
    position: relative;
    overflow: hidden;
  }

  .scrollbar {
    position: absolute;
  }

  .scroller {
    position: absolute;
  }

  .gutter-cell {
    width: 50px;
    font-family: monospace;
    position: absolute;
    text-align: right;
  }

  .container {
    position: absolute;
    top: 0;
    width: 1000000px;
    height: 1000000px;
  }

  textarea {
    opacity: 0;
    padding: 0;
    border: 0;
    margin: 0;

    width: 0;
    height: 0;
  }
</style>
