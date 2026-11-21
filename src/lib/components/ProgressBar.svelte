<script lang="ts">
  /**
   * Where the user stands in a subject: known (latest answer right, in the
   * subject's colour), mistakes (latest wrong, in the critical colour) and
   * new (the track). One bar, segments in that order.
   */
  import { fill, t } from "../i18n/index.svelte";
  import type { Progress } from "../types";

  let { progress, color, height = 6 }: { progress: Progress; color: string; height?: number } = $props();

  const total = $derived(Math.max(progress.questions, 1));
  const unseen = $derived(progress.questions - progress.correct - progress.wrong);
  const label = $derived(
    fill(t().progress.summary, { known: progress.correct, mistakes: progress.wrong, unseen: Math.max(unseen, 0) }),
  );
</script>

<div class="bar" style:height="{height}px" role="img" aria-label={label} title={label}>
  {#if progress.correct > 0}
    <span style:width="{(progress.correct / total) * 100}%" style:background={color}></span>
  {/if}
  {#if progress.wrong > 0}
    <span class="wrong" style:width="{(progress.wrong / total) * 100}%"></span>
  {/if}
</div>

<style>
  .bar {
    display: flex;
    gap: 2px;
    width: 100%;
    overflow: hidden;
    border-radius: 99px;
    background: var(--grid);
  }
  span {
    min-width: 3px;
    transition: width 0.3s;
  }
  span:first-child {
    border-radius: 99px 0 0 99px;
  }
  .wrong {
    background: var(--critical);
  }
  @media (prefers-reduced-motion: reduce) {
    span {
      transition: none;
    }
  }
</style>
