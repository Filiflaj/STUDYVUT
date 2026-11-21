<script lang="ts">
  /**
   * Answers per day as bars: the right ones in the full accent, the rest in
   * a lighter step of the same hue (an ordinal pair checked with the dataviz
   * validator in both themes), a dashed line at the daily goal, a tooltip
   * per day and a table for screen readers.
   */
  import { fmtDay } from "../format";
  import { t } from "../i18n/index.svelte";
  import { theme } from "../theme.svelte";
  import type { DayActivity } from "../types";

  let { days, goal }: { days: DayActivity[]; goal: number } = $props();

  /** Light and dark steps: [rest, right]. */
  const STEPS = { light: ["#86a0e8", "#2b55d0"], dark: ["#3d5394", "#7b97ff"] } as const;
  const steps = $derived(STEPS[theme.effective]);

  let width = $state(600);
  const H = 150;
  const PAD = { top: 10, right: 6, bottom: 22, left: 30 };

  const max = $derived.by(() => {
    const top = Math.max(goal, ...days.map((d) => d.answered), 1);
    const step = top <= 10 ? 2 : top <= 50 ? 10 : top <= 200 ? 50 : 100;
    return Math.ceil(top / step) * step;
  });
  const plotW = $derived(Math.max(width - PAD.left - PAD.right, 10));
  const plotH = H - PAD.top - PAD.bottom;
  const slot = $derived(plotW / Math.max(days.length, 1));
  const barW = $derived(Math.max(Math.min(slot * 0.62, 18), 2));
  const y = (v: number) => PAD.top + plotH - (v / max) * plotH;
  const ticks = $derived([0, max / 2, max]);

  let hover = $state<number | null>(null);
  const hovered = $derived(hover === null ? null : days[hover]);

  /** A bar with rounded data end (4 px) anchored to the baseline. */
  function bar(x: number, top: number, bottom: number, rounded: boolean): string {
    const h = bottom - top;
    if (h <= 0) return "";
    const r = rounded ? Math.min(4, barW / 2, h) : 0;
    return `M${x},${bottom} V${top + r} Q${x},${top} ${x + r},${top} H${x + barW - r} Q${x + barW},${top} ${x + barW},${top + r} V${bottom} Z`;
  }
</script>

<div class="chart" bind:clientWidth={width}>
  <div class="legend" aria-hidden="true">
    <span><i style:background={steps[1]}></i>{t().overview.right}</span>
    <span><i style:background={steps[0]}></i>{t().overview.answered}</span>
  </div>
  <svg {width} height={H} role="img" aria-label={t().overview.activity}>
    {#each ticks as v (v)}
      <line class="grid" x1={PAD.left} x2={width - PAD.right} y1={y(v)} y2={y(v)} />
      <text class="axis" x={PAD.left - 6} y={y(v) + 4} text-anchor="end">{v}</text>
    {/each}
    {#each days as d, i (d.day)}
      {@const x = PAD.left + i * slot + (slot - barW) / 2}
      {@const rightTop = y(d.correct)}
      {@const allTop = y(d.answered)}
      <!-- The rest above the right ones, with a 2 px surface gap between them. -->
      {#if d.answered > d.correct}
        <path d={bar(x, allTop, d.correct > 0 ? rightTop - 2 : y(0), true)} fill={steps[0]} />
      {/if}
      {#if d.correct > 0}
        <path d={bar(x, rightTop, y(0), d.answered === d.correct)} fill={steps[1]} />
      {/if}
      {#if i % 7 === (days.length - 1) % 7}
        <text class="axis" x={x + barW / 2} y={H - 6} text-anchor="middle">{fmtDay(d.day)}</text>
      {/if}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <rect
        class="hit"
        x={PAD.left + i * slot}
        y={PAD.top}
        width={slot}
        height={plotH}
        onmouseenter={() => (hover = i)}
        onmouseleave={() => (hover = null)}
      />
    {/each}
    {#if goal > 0 && goal <= max}
      <line class="goal" x1={PAD.left} x2={width - PAD.right} y1={y(goal)} y2={y(goal)} />
    {/if}
    {#if hover !== null}
      <line class="cursor" x1={PAD.left + hover * slot + slot / 2} x2={PAD.left + hover * slot + slot / 2} y1={PAD.top} y2={y(0)} />
    {/if}
  </svg>
  {#if hovered && hover !== null}
    {@const left = Math.min(Math.max(PAD.left + hover * slot + slot / 2, 70), width - 70)}
    <div class="tip" style:left="{left}px" role="tooltip">
      <div class="date">{fmtDay(hovered.day, { weekday: "short", day: "numeric", month: "numeric" })}</div>
      <div class="row"><i style:background={steps[0]}></i>{t().overview.answered}<b class="tabular">{hovered.answered}</b></div>
      <div class="row"><i style:background={steps[1]}></i>{t().overview.right}<b class="tabular">{hovered.correct}</b></div>
    </div>
  {/if}
  <table class="sr-only">
    <caption>{t().overview.activity}</caption>
    <tbody>
      {#each days as d (d.day)}
        <tr><th scope="row">{fmtDay(d.day)}</th><td>{d.answered} {t().overview.answered}</td><td>{d.correct} {t().overview.right}</td></tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .chart {
    position: relative;
    width: 100%;
  }
  svg {
    display: block;
    overflow: visible;
  }
  .grid {
    stroke: var(--grid);
    stroke-width: 1;
  }
  .axis {
    fill: var(--text-muted);
    font-size: 10.5px;
    font-variant-numeric: tabular-nums;
  }
  .goal {
    stroke: var(--text-muted);
    stroke-width: 1;
    stroke-dasharray: 4 4;
  }
  .cursor {
    stroke: var(--border-strong);
    stroke-width: 1;
    pointer-events: none;
  }
  .hit {
    fill: transparent;
  }
  .legend {
    display: flex;
    gap: 14px;
    margin-bottom: 6px;
    font-size: 12px;
    color: var(--text-2);
  }
  .legend span,
  .tip .row {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  i {
    display: inline-block;
    width: 10px;
    height: 10px;
    border-radius: 3px;
  }
  .tip {
    position: absolute;
    top: 18px;
    transform: translateX(-50%);
    min-width: 130px;
    padding: 8px 10px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
    font-size: 12.5px;
    pointer-events: none;
    z-index: 2;
  }
  .tip .date {
    margin-bottom: 4px;
    font-weight: 600;
  }
  .tip b {
    margin-left: auto;
    padding-left: 12px;
    font-weight: 600;
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip: rect(0 0 0 0);
    white-space: nowrap;
  }
</style>
