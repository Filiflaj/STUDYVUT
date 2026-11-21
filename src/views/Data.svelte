<script lang="ts">
  import { onMount } from "svelte";
  import { api, errorMessage } from "../lib/api";
  import Icon from "../lib/components/Icon.svelte";
  import SegmentedControl from "../lib/components/SegmentedControl.svelte";
  import { fmtDateTime, percent, plainText } from "../lib/format";
  import { fill, plural, t } from "../lib/i18n/index.svelte";
  import { live } from "../lib/live.svelte";
  import { nav } from "../lib/nav.svelte";
  import { subjectColor } from "../lib/palette";
  import { theme } from "../lib/theme.svelte";
  import type { QuestionRow } from "../lib/types";

  type State = "all" | "mistakes" | "unseen" | "known" | "due";

  let rows = $state<QuestionRow[]>([]);
  let error = $state<string | null>(null);
  let subject = $state<string>("");
  let state_ = $state<State>("all");
  let query = $state("");

  onMount(() => {
    api.questions().then((r) => (rows = r), (e) => (error = errorMessage(e)));
  });

  const visible = $derived(new Set(live.visibleSubjects.map((s) => s.id)));
  const today = $derived(live.overview?.today ?? 0);
  const texts = $derived(new Map(rows.map((r) => [`${r.subject}/${r.id}`, plainText(r.text)])));

  const filtered = $derived.by(() => {
    const q = query.trim().toLowerCase();
    return rows.filter((r) => {
      if (!visible.has(r.subject)) return false;
      if (subject && r.subject !== subject) return false;
      if (state_ === "mistakes" && r.last_correct !== false) return false;
      if (state_ === "unseen" && r.answered > 0) return false;
      if (state_ === "known" && r.last_correct !== true) return false;
      if (state_ === "due" && (r.due_day === null || r.due_day > today)) return false;
      if (q) {
        const hay = `${r.id} ${r.topic} ${texts.get(`${r.subject}/${r.id}`) ?? ""}`.toLowerCase();
        if (!hay.includes(q)) return false;
      }
      return true;
    });
  });

  /** Practise the shown questions: up to 50, of the first shown subject. */
  const practice = $derived.by(() => {
    const first = filtered[0]?.subject;
    return first ? filtered.filter((r) => r.subject === first).slice(0, 50) : [];
  });

  const STATES = $derived<{ value: State; label: string }[]>([
    { value: "all", label: t().data.all },
    { value: "mistakes", label: t().data.mistakes },
    { value: "due", label: t().data.due },
    { value: "unseen", label: t().data.unseen },
    { value: "known", label: t().data.known },
  ]);

  function review(r: QuestionRow): string {
    if (r.due_day === null) return "–";
    const d = r.due_day - today;
    return d <= 0 ? t().data.dueToday : plural(t().data.dueIn, d);
  }
</script>

<div class="view">
  <div class="page-head">
    <h1>{t().data.title}</h1>
    <span class="muted small">{plural(t().data.count, filtered.length)}</span>
  </div>

  <div class="filters">
    <select class="select" aria-label={t().data.subject} bind:value={subject}>
      <option value="">{t().data.allSubjects}</option>
      {#each live.visibleSubjects.filter((s) => s.progress.questions > 0) as s (s.id)}
        <option value={s.id}>{s.code} · {s.name}</option>
      {/each}
    </select>
    <SegmentedControl options={STATES} value={state_} label={t().data.state} onchange={(v) => (state_ = v)} />
    <label class="search">
      <Icon name="search" size={14} />
      <input class="input" type="search" placeholder={t().data.search} bind:value={query} />
    </label>
    {#if practice.length}
      <button
        class="btn primary"
        title={t().data.practiceHint}
        onclick={() =>
          nav.quiz(practice[0].subject, { kind: "questions", ids: practice.map((r) => r.id) }, t().quiz.selected)}
      >
        <Icon name="play" size={12} />
        {plural(t().data.practice, practice.length)}
      </button>
    {/if}
  </div>

  {#if error}
    <div class="card empty">{error}</div>
  {:else if filtered.length === 0}
    <div class="card empty">{t().data.empty}</div>
  {:else}
    <div class="card table-wrap">
      <table>
        <thead>
          <tr>
            <th>{t().data.colQuestion}</th>
            <th>{t().data.colTopic}</th>
            <th class="num">{t().data.colAnswers}</th>
            <th class="num">{t().data.colSuccess}</th>
            <th>{t().data.colLast}</th>
            <th>{t().data.colReview}</th>
          </tr>
        </thead>
        <tbody>
          {#each filtered.slice(0, 500) as r (`${r.subject}/${r.id}`)}
            {@const s = live.subject(r.subject)}
            {@const text = texts.get(`${r.subject}/${r.id}`) ?? ""}
            {@const pct = percent(r.correct, r.answered)}
            <tr>
              <td class="q">
                <span class="dot" style:background={subjectColor(s?.color, theme.effective === "dark")}></span>
                <span class="qtext">
                  <span class="id mono">{s?.code.replace(/^BPC-/, "")} {r.id}</span>
                  {text.length > 120 ? `${text.slice(0, 120)}…` : text}
                </span>
              </td>
              <td class="topic">{r.topic}</td>
              <td class="num tabular">{r.answered}</td>
              <td class="num tabular">
                {#if pct !== null}
                  <span class:bad={r.last_correct === false} class:ok={r.last_correct === true}>{pct} %</span>
                {:else}–{/if}
              </td>
              <td class="tabular muted">{r.last_ts ? fmtDateTime(r.last_ts) : "–"}</td>
              <td class="muted">{review(r)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    {#if filtered.length > 500}
      <p class="muted small">{fill("{shown} / {all}", { shown: 500, all: filtered.length })}</p>
    {/if}
  {/if}
</div>

<style>
  .small {
    font-size: 12.5px;
  }
  .filters {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 10px;
  }
  .search {
    position: relative;
    display: flex;
    align-items: center;
    flex: 1;
    min-width: 180px;
    color: var(--text-muted);
  }
  .search :global(svg) {
    position: absolute;
    left: 9px;
    pointer-events: none;
  }
  .search .input {
    width: 100%;
    padding-left: 30px;
  }
  .table-wrap {
    overflow-x: auto;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
  }
  th,
  td {
    padding: 8px 12px;
    border-bottom: 1px solid var(--border);
    text-align: left;
    vertical-align: top;
  }
  th {
    position: sticky;
    top: 0;
    background: var(--surface);
    font-size: 12px;
    font-weight: 600;
    color: var(--text-2);
    white-space: nowrap;
  }
  tbody tr:last-child td {
    border-bottom: none;
  }
  tbody tr:hover {
    background: var(--surface-2);
  }
  .num {
    text-align: right;
    white-space: nowrap;
  }
  .q {
    display: flex;
    gap: 8px;
    min-width: 320px;
  }
  .qtext {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .id {
    font-size: 11px;
    color: var(--text-muted);
  }
  .dot {
    flex: none;
    width: 8px;
    height: 8px;
    margin-top: 5px;
    border-radius: 50%;
  }
  .topic {
    min-width: 140px;
    color: var(--text-2);
  }
  .ok {
    color: var(--good-text);
  }
  .bad {
    color: var(--critical);
  }
</style>
