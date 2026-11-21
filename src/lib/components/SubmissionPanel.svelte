<script lang="ts">
  /**
   * A submitted work on the Study page (docs/submissions.md): what it is,
   * its files (documents open in their system app, everything can be shown
   * in its folder or saved), what it was about, and links to the study text
   * and the quiz topics it practises.
   */
  import { api, errorMessage } from "../api";
  import { fill, i18n, plural, t } from "../i18n/index.svelte";
  import { render } from "../markdown";
  import { nav } from "../nav.svelte";
  import type { SubmissionDetail } from "../types";
  import Icon from "./Icon.svelte";
  import Prose from "./Prose.svelte";

  let { subject, id }: { subject: string; id: string } = $props();

  let detail = $state<SubmissionDetail | null>(null);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);
  const s = $derived(t().submissions);

  $effect(() => {
    const sub = subject;
    const work = id;
    detail = null;
    error = null;
    notice = null;
    let cancelled = false;
    api.submission(sub, work).then(
      (d) => {
        if (!cancelled) detail = d;
      },
      (e) => {
        if (!cancelled) error = errorMessage(e);
      },
    );
    return () => {
      cancelled = true;
    };
  });

  const summary = $derived(detail?.summary ? render(detail.summary, { subject }).html : "");

  /** `2026-10-08` in the UI language. */
  function date(iso: string): string {
    const [y, m, d] = iso.split("-").map(Number);
    return new Date(y, m - 1, d).toLocaleDateString(i18n.locale, { day: "numeric", month: "numeric", year: "numeric" });
  }

  function size(bytes: number): string {
    if (bytes < 1024 * 1024) return `${Math.max(1, Math.round(bytes / 1024))} kB`;
    return `${(bytes / 1024 / 1024).toFixed(1).replace(".", i18n.lang === "cs" ? "," : ".")} MB`;
  }

  async function act(action: () => Promise<void>) {
    notice = null;
    try {
      await action();
    } catch (e) {
      notice = fill(s.failed, { error: errorMessage(e) });
    }
  }

  function practise(d: SubmissionDetail) {
    const ids = [...d.practice];
    for (let i = ids.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [ids[i], ids[j]] = [ids[j], ids[i]];
    }
    nav.quiz(d.subject, { kind: "questions", ids }, fill(s.practiceName, { title: d.title }));
  }
</script>

{#if error}
  <div class="card empty">{fill(s.loadFailed, { error })}</div>
{:else if detail}
  {@const d = detail}
  <div class="card body">
    <div class="kicker">
      {s.kinds[d.kind]}{#if d.task}&ensp;·&ensp;{fill(s.task, { task: d.task })}{/if}
    </div>
    <h2 class="title">{d.title}</h2>
    {#if d.measured || d.submitted}
      <div class="meta">
        {#if d.measured}<span>{s.measured} {date(d.measured)}</span>{/if}
        {#if d.submitted}<span>{s.submitted} {date(d.submitted)}</span>{/if}
      </div>
    {/if}

    <ul class="files">
      {#each d.files as f (f.index)}
        <li>
          <Icon name="file-text" size={16} />
          <span class="name">
            <span class="ell">{f.label ?? f.path}</span>
            <span class="muted small">{s.roles[f.role]} · {size(f.size)}</span>
          </span>
          {#if f.openable}
            <button class="btn" onclick={() => act(() => api.openSubmissionFile(d.subject, d.id, f.index))}>
              <Icon name="external" size={12} />
              {s.open}
            </button>
          {/if}
          <button
            class="btn ghost icon"
            title={s.reveal}
            aria-label={s.reveal}
            onclick={() => act(() => api.revealSubmissionFile(d.subject, d.id, f.index))}
          >
            <Icon name="folder" size={14} />
          </button>
          <button
            class="btn ghost icon"
            title={s.save}
            aria-label={s.save}
            onclick={() =>
              act(async () => {
                const path = await api.saveSubmissionFile(d.subject, d.id, f.index, i18n.lang);
                if (path) notice = fill(s.saved, { path });
              })}
          >
            <Icon name="download" size={14} />
          </button>
        </li>
      {/each}
    </ul>
    {#if notice}<p class="muted small">{notice}</p>{/if}

    {#if summary}
      <h3>{s.summary}</h3>
      <Prose html={summary} />
    {/if}
  </div>

  {#if d.chapters.length || d.practice.length}
    <div class="foot">
      {#each d.chapters as c (c.id)}
        <button class="btn ghost" onclick={() => nav.study(d.subject, c.id)}>
          <Icon name="book" size={14} />
          <span class="ell">{c.title}</span>
        </button>
      {/each}
      {#if d.practice.length}
        <button class="btn primary" onclick={() => practise(d)}>
          <Icon name="play" size={12} />
          {s.practice} · {plural(t().study.practiceCount, d.practice.length)}
        </button>
      {/if}
    </div>
  {/if}
{:else}
  <div class="card empty">{t().common.loading}</div>
{/if}

<style>
  .body {
    padding: 18px 28px 24px;
  }
  .title {
    margin-bottom: 6px;
    font-size: 20px;
    letter-spacing: -0.01em;
  }
  .kicker {
    font-size: 12px;
    color: var(--text-muted);
  }
  .ell {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .foot .btn.ghost {
    max-width: 100%;
    min-width: 0;
    color: var(--text-2);
  }
  .meta {
    display: flex;
    flex-wrap: wrap;
    gap: 14px;
    margin-top: 4px;
    font-size: 12.5px;
    color: var(--text-muted);
  }
  .files {
    list-style: none;
    margin: 16px 0 8px;
    padding: 0;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
  }
  .files li {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
  }
  .files li + li {
    border-top: 1px solid var(--border);
  }
  .name {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }
  .small {
    font-size: 12px;
  }
  h3 {
    margin: 18px 0 6px;
    font-size: 14px;
  }
  .foot {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    justify-content: flex-end;
  }
</style>
