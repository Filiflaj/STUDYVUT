<script lang="ts">
  /** One question of the running quiz and, once answered, its feedback (as the BFY1 trainer). */
  import { contentUrl } from "../api";
  import { fmtNumber, letter, parseNumber } from "../format";
  import { fill, t } from "../i18n/index.svelte";
  import { cleanSvg, render, renderInline } from "../markdown";
  import { quiz } from "../quiz.svelte";
  import Icon from "./Icon.svelte";
  import Prose from "./Prose.svelte";

  let { color }: { color: string } = $props();

  const q = $derived(quiz.question);
  const ctx = $derived({ subject: quiz.payload?.subject, svgs: quiz.payload?.svgs ?? {} });
  const text = $derived(q ? render(q.text, ctx).html : "");
  const figure = $derived.by(() => {
    if (!q?.figure) return "";
    const svg = ctx.svgs[q.figure];
    if (svg !== undefined) return cleanSvg(svg, q.topic);
    return `<img src="${contentUrl(`${ctx.subject}/${q.figure}`)}" alt="" />`;
  });
  // In the order shown (quiz.order); `i` is the option's index in the content.
  const options = $derived(
    q?.kind === "choice" ? quiz.order.map((i) => ({ i, html: renderInline(q.options[i] ?? "", ctx) })) : [],
  );
  const notes = $derived(q?.kind === "choice" && q.notes ? q.notes.map((n) => renderInline(n, ctx)) : null);
  const explanation = $derived(q ? render(q.explanation, ctx).html : "");
  const warning = $derived(q?.warning ? renderInline(q.warning, ctx) : "");

  let numeric = $state("");
  let numericError = $state(false);
  let input = $state<HTMLInputElement>();
  let nextButton = $state<HTMLButtonElement>();

  // A new question: empty input, focus where the keyboard is needed.
  $effect(() => {
    void quiz.index;
    void quiz.payload;
    numeric = "";
    numericError = false;
    queueMicrotask(() => input?.focus({ preventScroll: true }));
  });

  $effect(() => {
    if (quiz.answer) queueMicrotask(() => nextButton?.focus({ preventScroll: true }));
  });

  function submitNumeric(e: SubmitEvent) {
    e.preventDefault();
    if (quiz.answer) {
      void quiz.next();
      return;
    }
    const value = parseNumber(numeric);
    if (Number.isNaN(value)) {
      numericError = true;
      return;
    }
    numericError = false;
    void quiz.respond({ kind: "numeric", value });
  }

  const chosen = $derived(quiz.answer?.response.kind === "choice" ? quiz.answer.response.index : null);
  const typed = $derived(quiz.answer?.response.kind === "numeric" ? quiz.answer.response.value : null);
</script>

{#if q}
  <div class="card qcard">
    <div class="top">
      <div class="meta">
        <span class="chip topic" style:--c={color}>{q.topic}</span>
        {#if q.source}<span class="chip src">{q.source}</span>{/if}
      </div>
      <div class="count tabular">
        {fill(t().quiz.question, { i: quiz.index + 1, n: quiz.total })} · {fill(t().quiz.score, { n: quiz.score })}
      </div>
    </div>
    <div class="track" aria-hidden="true">
      <div style:width="{((quiz.index + (quiz.answer ? 1 : 0)) / quiz.total) * 100}%"></div>
    </div>

    <Prose html={text} class="qtext" />
    {#if figure}<div class="figbox">{@html figure}</div>{/if}

    {#if q.kind === "choice"}
      <div class="opts" class:grid={q.grid} role="group">
        {#each options as { i, html }, position (i)}
          {@const isRight = quiz.answer && i === q.answer}
          {@const isChosen = quiz.answer && i === chosen && i !== q.answer}
          <button
            class="opt"
            class:ok={isRight}
            class:bad={isChosen}
            class:dim={quiz.answer && !isRight && !isChosen}
            disabled={quiz.answer !== null || quiz.busy}
            onclick={() => quiz.respond({ kind: "choice", index: i })}
          >
            <span class="k">{letter(position)})</span>
            <span class="tx selectable">{@html html}</span>
            <span class="mark" aria-hidden="true">
              {#if isRight}<Icon name="check" size={16} stroke={2.6} />{:else if isChosen}<Icon name="close" size={16} stroke={2.6} />{/if}
            </span>
          </button>
        {/each}
      </div>
    {:else}
      <form class="num" onsubmit={submitNumeric} autocomplete="off">
        <label for="numeric-answer" class="hint">{t().quiz.numericLabel}</label>
        <div class="row">
          <input
            id="numeric-answer"
            class="input"
            class:ok={quiz.answer?.verdict.correct}
            class:bad={quiz.answer && !quiz.answer.verdict.correct}
            inputmode="decimal"
            bind:value={numeric}
            bind:this={input}
            readonly={quiz.answer !== null}
          />
          {#if q.unit}<span class="unit">{q.unit}</span>{/if}
          {#if !quiz.answer}<button class="btn primary" type="submit" disabled={quiz.busy}>{t().quiz.check}</button>{/if}
        </div>
        {#if numericError}<span class="err">{t().quiz.enterNumber}</span>{/if}
      </form>
    {/if}

    {#if quiz.answer}
      {@const ok = quiz.answer.verdict.correct}
      <div class="fb" class:ok class:bad={!ok} role="status">
        {#if q.kind === "choice"}
          <h3>{ok ? t().quiz.correct : fill(t().quiz.wrong, { key: letter(quiz.shown(q.answer)) })}</h3>
          {#if !ok && chosen !== null && notes?.[chosen]}
            <p><strong>{fill(t().quiz.whyNot, { key: letter(quiz.shown(chosen)) })}</strong>: {@html notes[chosen]}</p>
          {/if}
          <div class="why">
            <strong>{t().quiz.explanation}:</strong>
            <Prose html={explanation} />
          </div>
        {:else}
          <h3>
            {ok ? t().quiz.correct : fill(t().quiz.wrongNumeric, { value: fmtNumber(q.value), unit: q.unit })}
          </h3>
          <p>
            {fill(t().quiz.yourAnswer, { value: fmtNumber(typed ?? 0), unit: q.unit })}
            {#if !ok && q.tolerance > 0}{fill(t().quiz.tolerance, { tol: fmtNumber(q.tolerance) })}{/if}
          </p>
          {#if quiz.answer.verdict.sign_only}
            <p><strong>{t().quiz.signOnly}</strong> {q.sign_hint ?? ""}</p>
          {/if}
          <div class="why">
            <strong>{t().quiz.procedure}:</strong>
            <Prose html={explanation} />
          </div>
        {/if}
        {#if warning}<p class="warn"><Icon name="warning" size={14} /> <span>{@html warning}</span></p>{/if}
        {#if q.kind === "choice" && notes}
          <div class="notes-h">{t().quiz.allOptions}</div>
          <ul class="notes">
            {#each quiz.order as i, position (i)}
              {@const note = notes[i]}
              {#if i === q.answer || note}
                <li class:yes={i === q.answer} class:chosen={i === chosen && i !== q.answer}>
                  <span class="k">{letter(position)})</span>
                  <span class="selectable">{#if i === q.answer}{t().quiz.rightOption}{:else}{@html note}{/if}</span>
                </li>
              {/if}
            {/each}
          </ul>
        {/if}
      </div>
      <div class="actions">
        <button class="btn primary" bind:this={nextButton} disabled={quiz.busy} onclick={() => quiz.next()}>
          {quiz.last ? t().quiz.showResult : t().quiz.next}
        </button>
        <span class="hint"><kbd>Enter</kbd></span>
      </div>
    {:else if q.kind === "choice"}
      <div class="hint">
        {fill(t().quiz.keysHint, { last: letter(q.options.length - 1) })}
      </div>
    {:else}
      <div class="hint">{t().quiz.enterHint}</div>
    {/if}
  </div>
{/if}

<style>
  .qcard {
    display: flex;
    flex-direction: column;
    gap: 14px;
    padding: 20px clamp(16px, 3vw, 24px);
    min-width: 0;
  }
  .top {
    display: flex;
    flex-wrap: wrap;
    gap: 8px 14px;
    align-items: center;
    justify-content: space-between;
  }
  .meta {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
  .chip {
    padding: 2px 9px;
    border-radius: 99px;
    font-size: 12px;
    font-weight: 500;
    background: var(--surface-2);
    color: var(--text-2);
    border: 1px solid var(--border);
  }
  .chip.topic {
    background: color-mix(in srgb, var(--c) 12%, transparent);
    border-color: transparent;
    color: var(--text);
  }
  .count {
    font-size: 12.5px;
    color: var(--text-muted);
  }
  .track {
    height: 4px;
    border-radius: 99px;
    background: var(--grid);
    overflow: hidden;
  }
  .track > div {
    height: 100%;
    background: var(--accent);
    transition: width 0.3s;
  }
  :global(.qtext) {
    font-size: 15.5px;
  }
  :global(.qtext p) {
    color: var(--text);
  }
  .figbox {
    padding: 12px;
    border: 1px dashed var(--border-strong);
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    overflow-x: auto;
  }
  .figbox :global(img) {
    display: block;
    max-width: 100%;
    margin: 0 auto;
  }
  .opts {
    display: grid;
    gap: 8px;
  }
  .opts.grid {
    grid-template-columns: repeat(auto-fit, minmax(min(100%, 240px), 1fr));
  }
  .opt {
    display: flex;
    gap: 12px;
    align-items: baseline;
    width: 100%;
    padding: 10px 14px;
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-sm);
    background: var(--surface);
    text-align: left;
    line-height: 1.45;
    cursor: pointer;
    min-width: 0;
  }
  .opt:hover:not(:disabled) {
    border-color: var(--accent);
    background: var(--accent-weak);
  }
  .opt:disabled {
    cursor: default;
  }
  .opt .k {
    flex: 0 0 1.6em;
    color: var(--text-muted);
    font-weight: 600;
  }
  .opt .tx {
    min-width: 0;
    flex: 1;
  }
  .opt .tx :global(.fig-inline svg) {
    display: inline-block;
    max-width: 200px;
    vertical-align: middle;
  }
  .opt .mark {
    flex: none;
    display: flex;
    align-self: center;
  }
  .opt.ok {
    border-color: var(--good);
    background: var(--good-weak);
  }
  .opt.ok .mark {
    color: var(--good-text);
  }
  .opt.bad {
    border-color: var(--critical);
    background: var(--critical-weak);
  }
  .opt.bad .mark {
    color: var(--critical);
  }
  .opt.dim {
    opacity: 0.55;
  }
  .num {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .num .row {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }
  .num .input {
    width: 180px;
    height: 34px;
    font-size: 15px;
    font-variant-numeric: tabular-nums;
  }
  .num .input.ok {
    border-color: var(--good);
    background: var(--good-weak);
  }
  .num .input.bad {
    border-color: var(--critical);
    background: var(--critical-weak);
  }
  .unit {
    color: var(--text-2);
    font-weight: 500;
  }
  .err {
    color: var(--critical);
    font-size: 12.5px;
  }
  .fb {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 14px 16px;
    border-radius: var(--radius-sm);
    border: 1px solid;
  }
  .fb.ok {
    border-color: var(--good);
    background: var(--good-weak);
  }
  .fb.bad {
    border-color: var(--critical);
    background: var(--critical-weak);
  }
  .fb h3 {
    font-size: 15px;
  }
  .fb.ok h3 {
    color: var(--good-text);
  }
  .fb.bad h3 {
    color: var(--critical);
  }
  .fb p {
    margin: 0;
  }
  .why :global(.prose) {
    margin-top: 2px;
  }
  .warn {
    display: flex;
    gap: 6px;
    align-items: baseline;
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    background: var(--warning-weak);
    color: var(--warning-text);
  }
  .notes-h {
    padding-top: 8px;
    border-top: 1px solid var(--border);
    font-size: 11.5px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
  }
  .notes {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 5px;
    font-size: 13.5px;
  }
  .notes li {
    display: flex;
    gap: 10px;
    align-items: baseline;
    color: var(--text-2);
  }
  .notes .k {
    flex: 0 0 1.8em;
    font-weight: 600;
    color: var(--text-muted);
  }
  .notes .yes {
    color: var(--good-text);
  }
  .notes .chosen {
    color: var(--critical);
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .hint {
    font-size: 12.5px;
    color: var(--text-muted);
  }
  kbd {
    display: inline-block;
    padding: 0 6px;
    border: 1px solid var(--border-strong);
    border-bottom-width: 2px;
    border-radius: 5px;
    background: var(--surface-2);
    font-family: var(--font);
    font-size: 11.5px;
    line-height: 18px;
  }
  @media (prefers-reduced-motion: reduce) {
    .track > div {
      transition: none;
    }
  }
</style>
