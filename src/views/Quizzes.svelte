<script lang="ts">
  import Icon from "../lib/components/Icon.svelte";
  import QuestionCard from "../lib/components/QuestionCard.svelte";
  import SubjectSelect from "../lib/components/SubjectSelect.svelte";
  import { percent, plainText } from "../lib/format";
  import { fill, plural, t } from "../lib/i18n/index.svelte";
  import { live } from "../lib/live.svelte";
  import { nav } from "../lib/nav.svelte";
  import { subjectColor } from "../lib/palette";
  import { quiz } from "../lib/quiz.svelte";
  import { setDescription, setName } from "../lib/sets";
  import { theme } from "../lib/theme.svelte";
  import type { SetView } from "../lib/types";

  const subject = $derived(live.subject(nav.subject) ?? live.defaultSubject);
  const color = $derived(subjectColor(subject?.color, theme.effective === "dark"));
  const sets = $derived(subject?.sets ?? []);
  /** The running quiz belongs to the subject on screen. */
  const mine = $derived(quiz.active && quiz.payload?.subject === subject?.id);
  const running = $derived(quiz.active ? live.subject(quiz.payload?.subject) : undefined);

  // A quiz another page asked for (Dashboard, a chapter, Statistics).
  $effect(() => {
    const r = nav.request;
    if (!r) return;
    nav.request = null;
    void quiz.start(r.subject, r.spec, r.name);
  });

  function start(set: SetView) {
    if (subject) void quiz.start(subject.id, set.spec);
  }

  const result = $derived(quiz.result);
  const pct = $derived(result ? (percent(result.correct, result.total) ?? 0) : 0);
  const message = $derived(
    pct >= 90 ? t().quiz.msgGreat : pct >= 70 ? t().quiz.msgGood : pct >= 50 ? t().quiz.msgHalf : t().quiz.msgLow,
  );
  const missed = $derived(
    quiz.missed.map((id) => quiz.payload?.questions.find((q) => q.id === id)).filter((q) => q !== undefined),
  );

  function onkeydown(e: KeyboardEvent) {
    if (e.metaKey || e.ctrlKey || e.altKey || !mine || quiz.done) return;
    const tag = (e.target as HTMLElement | null)?.tagName?.toLowerCase() ?? "";
    const q = quiz.question;
    if (!q) return;
    if (quiz.answer && (e.key === "Enter" || e.key === "ArrowRight")) {
      if (tag === "button" && e.key === "Enter") return; // the focused button clicks itself
      e.preventDefault();
      void quiz.next();
      return;
    }
    if (quiz.answer || tag === "input" || tag === "textarea" || tag === "select" || q.kind !== "choice") return;
    const k = e.key.toLowerCase();
    let i = "abcdefgh".indexOf(k);
    if (i < 0 && /^[1-8]$/.test(k)) i = Number(k) - 1;
    if (i >= 0 && i < q.options.length) {
      e.preventDefault();
      void quiz.choose(i);
    }
  }
</script>

<svelte:window {onkeydown} />

<div class="view">
  {#if subject}
    <button class="back" onclick={() => subject && nav.open(subject.id)}>
      <Icon name="chevron-left" size={14} />
      {subject.code} · {subject.name}
    </button>
  {/if}
  <div class="page-head">
    <h1>
      {t().quiz.title}
      {#if subject}<span class="sub">· {subject.code} {subject.name}</span>{/if}
    </h1>
    <div class="head-tools">
      {#if mine && !quiz.done}
        <button class="btn ghost" onclick={() => quiz.close()}><Icon name="close" size={13} /> {t().quiz.end}</button>
      {/if}
      <SubjectSelect onchange={(id) => nav.quiz(id)} value={subject?.id} />
    </div>
  </div>

  {#if !subject}
    <div class="card empty">{t().overview.hiddenAll}</div>
  {:else if subject.questions === 0}
    <div class="card empty">
      <p>{t().quiz.noQuizzes}</p>
      <p class="muted small">{fill(t().overview.noContentHint, { id: subject.id })}</p>
    </div>
  {:else}
    <section>
      <h2 class="section-title">{t().quiz.sets}</h2>
      <div class="sets">
        {#each sets as set (set.key)}
          <button
            class="set"
            class:on={mine && quiz.payload?.key === set.key}
            disabled={set.count === 0 || quiz.busy}
            onclick={() => start(set)}
          >
            <span class="name">{setName(subject, set.key)}</span>
            <span class="desc">
              {plural(t().quiz.questions, set.count)}{#if setDescription(set)}&nbsp;· {setDescription(set)}{/if}
            </span>
          </button>
        {/each}
      </div>
    </section>

    {#if quiz.error === "empty_set"}
      <div class="card empty">{t().quiz.emptySet}</div>
    {:else if quiz.error}
      <div class="card empty error">{fill(t().quiz.startFailed, { error: quiz.error })}</div>
    {/if}

    {#if mine && result}
      <div class="card end">
        <div class="eyebrow">{setName(subject, quiz.payload?.key ?? "", quiz.name)} · {t().quiz.result}</div>
        <div class="big tabular">{result.correct} / {result.total}</div>
        <p>{pct} % · {message}</p>
        {#if missed.length}
          <h2>{t().quiz.missed}</h2>
          <ul class="missed">
            {#each missed as q (q.id)}
              {@const text = plainText(q.text)}
              <li>
                <span class="muted small">{q.source ? `${q.source} · ` : ""}{q.topic}</span>
                {text.length > 160 ? `${text.slice(0, 160)}…` : text}
              </li>
            {/each}
          </ul>
        {/if}
        <div class="actions">
          {#if missed.length}
            <button class="btn primary" onclick={() => quiz.retryMissed()}>{t().quiz.retryMissed}</button>
          {/if}
          <button class="btn" onclick={() => quiz.again()}>{t().quiz.again}</button>
          <button class="btn ghost" onclick={() => quiz.close()}>{t().common.close}</button>
        </div>
      </div>
    {:else if mine}
      <QuestionCard {color} />
    {:else if running && quiz.payload && !quiz.done}
      <button class="card resume" onclick={() => nav.quiz(running.id)}>
        <Icon name="play" size={13} />
        {running.code}: {setName(running, quiz.payload.key, quiz.name)} ({quiz.index + 1} / {quiz.total})
      </button>
    {:else if !quiz.error}
      <p class="muted hint">{t().quiz.pickSet}</p>
    {/if}
  {/if}
</div>

<style>
  .back {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    align-self: flex-start;
    margin: -6px 0 -8px -6px;
    padding: 3px 6px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    font-size: 12.5px;
    cursor: pointer;
  }
  .back:hover {
    background: var(--hover);
    color: var(--text);
  }
  .sub {
    color: var(--text-muted);
    font-weight: 500;
  }
  .head-tools {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .section-title {
    padding: 0 4px;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-2);
  }
  .sets {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(190px, 1fr));
    gap: 10px;
  }
  .set {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    padding: 11px 13px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--surface);
    box-shadow: var(--shadow);
    text-align: left;
    cursor: pointer;
  }
  .set:hover:not(:disabled) {
    border-color: var(--accent);
  }
  .set.on {
    border-color: var(--accent);
    background: var(--accent-weak);
  }
  .set:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .set .name {
    font-weight: 600;
  }
  .set .desc {
    font-size: 12.5px;
    color: var(--text-muted);
  }
  .empty p {
    margin: 0 0 4px;
  }
  .error {
    color: var(--critical);
  }
  .small {
    font-size: 12.5px;
  }
  .hint {
    padding: 0 4px;
  }
  .end {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 20px 24px;
  }
  .eyebrow {
    font-size: 11.5px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
  }
  .big {
    font-size: 44px;
    font-weight: 700;
    line-height: 1;
  }
  .end p {
    margin: 0;
  }
  .end h2 {
    margin-top: 8px;
    font-size: 14px;
  }
  .missed {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .missed li {
    display: flex;
    flex-direction: column;
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    background: var(--critical-weak);
    font-size: 13.5px;
  }
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-top: 6px;
  }
  .resume {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    color: var(--text);
    cursor: pointer;
    text-align: left;
  }
  .resume:hover {
    border-color: var(--accent);
  }
</style>
