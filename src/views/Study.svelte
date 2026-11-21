<script lang="ts">
  import { tick } from "svelte";
  import { api, errorMessage } from "../lib/api";
  import Icon from "../lib/components/Icon.svelte";
  import Prose from "../lib/components/Prose.svelte";
  import SegmentedControl from "../lib/components/SegmentedControl.svelte";
  import SubjectSelect from "../lib/components/SubjectSelect.svelte";
  import SubmissionPanel from "../lib/components/SubmissionPanel.svelte";
  import { fill, plural, t } from "../lib/i18n/index.svelte";
  import { live } from "../lib/live.svelte";
  import { render, type Rendered } from "../lib/markdown";
  import { nav } from "../lib/nav.svelte";
  import type { ChapterPayload, SubmissionRow } from "../lib/types";

  /** The chosen subject, or the first visible one. */
  const subject = $derived(live.subject(nav.subject) ?? live.defaultSubject);
  const chapters = $derived(subject?.chapters ?? []);
  const chapterId = $derived(
    chapters.some((c) => c.id === nav.chapter) ? nav.chapter : (chapters[0]?.id ?? null),
  );
  const index = $derived(chapters.findIndex((c) => c.id === chapterId));

  /** The subject's submitted work (docs/submissions.md), listed under the chapters. */
  let works = $state<SubmissionRow[]>([]);
  $effect(() => {
    const s = subject;
    works = [];
    if (!s || s.submissions === 0) return;
    let cancelled = false;
    api.submissions(s.id).then(
      (rows) => {
        if (!cancelled) works = rows;
      },
      (e) => console.error(e),
    );
    return () => {
      cancelled = true;
    };
  });
  /** The open work: the chosen one, or the first when the subject has no chapters. */
  const workId = $derived(
    works.some((w) => w.id === nav.submission)
      ? nav.submission
      : chapters.length === 0
        ? (works[0]?.id ?? null)
        : null,
  );

  let chapter = $state<ChapterPayload | null>(null);
  let rendered = $state<Rendered>({ html: "", sections: [] });
  let error = $state<string | null>(null);
  let article = $state<HTMLElement>();
  let active = $state("");

  $effect(() => {
    const s = subject?.id;
    const c = workId ? null : chapterId;
    error = null;
    if (!s || !c) {
      chapter = null;
      rendered = { html: "", sections: [] };
      return;
    }
    let cancelled = false;
    api.chapter(s, c).then(
      async (payload) => {
        if (cancelled) return;
        chapter = payload;
        // The title is the page's heading already.
        const md = payload.markdown.replace(/^# .+\n+/, "");
        rendered = render(md, { subject: s, svgs: payload.svgs });
        await tick();
        article?.closest("main")?.scrollTo({ top: 0 });
        spy();
      },
      (e) => {
        if (!cancelled) error = errorMessage(e);
      },
    );
    return () => {
      cancelled = true;
    };
  });

  /** Highlight the section being read. */
  function spy() {
    const root = article?.closest("main");
    const top = (root?.getBoundingClientRect().top ?? 0) + 80;
    let current = rendered.sections[0]?.id ?? "";
    for (const s of rendered.sections) {
      const h = article?.querySelector<HTMLElement>(`#${CSS.escape(s.id)}`);
      if (h && h.getBoundingClientRect().top <= top) current = s.id;
    }
    active = current;
  }

  $effect(() => {
    const main = article?.closest("main");
    main?.addEventListener("scroll", spy, { passive: true });
    return () => main?.removeEventListener("scroll", spy);
  });

  function show(id: string) {
    article?.querySelector(`#${CSS.escape(id)}`)?.scrollIntoView({ behavior: "smooth", block: "start" });
  }

  function open(id: string) {
    if (subject) nav.study(subject.id, id);
  }
</script>

<div class="view">
  {#if subject}
    <button class="back" onclick={() => subject && nav.open(subject.id)}>
      <Icon name="chevron-left" size={14} />
      {subject.code} · {subject.name}
    </button>
  {/if}
  <div class="page-head">
    <h1>
      {#if subject}<span class="code">{subject.code}</span> {subject.name}{:else}{t().study.title}{/if}
    </h1>
    <SubjectSelect onchange={(id) => nav.study(id)} value={subject?.id} />
  </div>

  {#if !subject}
    <div class="card empty">{t().overview.hiddenAll}</div>
  {:else if chapters.length === 0 && subject.submissions === 0}
    <div class="card empty">
      <p>{t().study.empty}</p>
      <p class="muted small">{fill(t().study.emptyHint, { id: subject.id })}</p>
    </div>
  {:else}
    <div class="layout">
      <nav class="toc" aria-label={t().study.chapters}>
        {#if works.length && chapters.length}
          <div class="switch">
            <SegmentedControl
              options={[
                { value: "chapters", label: t().study.chapters },
                { value: "works", label: t().submissions.title },
              ]}
              value={workId ? "works" : "chapters"}
              label={t().study.chapters}
              onchange={(v) => subject && (v === "works" ? nav.work(subject.id, works[0].id) : nav.study(subject.id))}
            />
          </div>
        {/if}
        {#if !workId}
        {#if !works.length}<div class="toc-title">{t().study.chapters}</div>{/if}
        {#each chapters as c (c.id)}
          <a
            href="#{c.id}"
            class:active={c.id === chapterId}
            aria-current={c.id === chapterId ? "page" : undefined}
            onclick={(e) => {
              e.preventDefault();
              open(c.id);
            }}>{c.title}</a
          >
          {#if c.id === chapterId && rendered.sections.length > 1}
            {#each rendered.sections as s (s.id)}
              <a
                href="#{s.id}"
                class="sub"
                class:deeper={s.level > 2}
                class:reading={active === s.id}
                onclick={(e) => {
                  e.preventDefault();
                  show(s.id);
                }}>{s.title}</a
              >
            {/each}
          {/if}
        {/each}
        {:else}
          {#if !chapters.length}<div class="toc-title">{t().submissions.title}</div>{/if}
          {#each works as w (w.id)}
            <a
              href="#{w.id}"
              class:active={w.id === workId}
              aria-current={w.id === workId ? "page" : undefined}
              onclick={(e) => {
                e.preventDefault();
                if (subject) nav.work(subject.id, w.id);
              }}
              >{#if w.task}<span class="task">{w.task}</span>{/if}{w.title}</a
            >
          {/each}
        {/if}
      </nav>

      <div class="column">
        {#if workId && subject}
          <SubmissionPanel subject={subject.id} id={workId} />
        {:else if error}
          <div class="card empty">{fill(t().study.loadFailed, { error })}</div>
        {:else if chapter}
          <div class="card body">
            <h2 class="title">{chapter.title}</h2>
            <Prose html={rendered.html} tag="article" bind:article />
          </div>
          <div class="foot">
            {#if index > 0}
              <button class="btn ghost" onclick={() => open(chapters[index - 1].id)}>
                <Icon name="chevron-left" size={14} />
                <span class="ell">{chapters[index - 1].title}</span>
              </button>
            {:else}<span></span>{/if}
            {#if chapter.questions > 0}
              <button
                class="btn primary"
                onclick={() =>
                  subject &&
                  chapter &&
                  nav.quiz(subject.id, { kind: "chapter", id: chapter.id }, fill(t().quiz.chapter, { title: chapter.title }))}
              >
                <Icon name="play" size={12} />
                {t().study.practice} · {plural(t().study.practiceCount, chapter.questions)}
              </button>
            {/if}
            {#if index < chapters.length - 1}
              <button class="btn ghost" onclick={() => open(chapters[index + 1].id)}>
                <span class="ell">{chapters[index + 1].title}</span>
                <Icon name="chevron-right" size={14} />
              </button>
            {:else}<span></span>{/if}
          </div>
        {:else}
          <div class="card empty">{t().common.loading}</div>
        {/if}
      </div>
    </div>
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
  .code {
    color: var(--text-muted);
    font-weight: 500;
  }
  .layout {
    display: grid;
    grid-template-columns: 220px minmax(0, 1fr);
    gap: 20px;
    align-items: start;
  }
  .toc {
    position: sticky;
    top: 18px;
    display: flex;
    flex-direction: column;
    gap: 1px;
    max-height: calc(100vh - 60px);
    overflow-y: auto;
    font-size: 13px;
  }
  .toc-title {
    padding: 0 10px 6px;
    font-size: 11.5px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
  }
  .toc a {
    padding: 5px 10px;
    border-radius: var(--radius-sm);
    color: var(--text-2);
    text-decoration: none;
    line-height: 1.3;
  }
  .toc a:hover {
    background: var(--hover);
    color: var(--text);
  }
  /* Colour and a bar, never a heavier weight (bolder text is wider and reflows). */
  .toc a.active {
    color: var(--text);
    background: var(--surface);
    box-shadow: var(--shadow), inset 0 0 0 1px var(--border), inset 2px 0 0 var(--accent);
  }
  .toc a.sub {
    padding: 3px 10px 3px 22px;
    font-size: 12.5px;
    color: var(--text-muted);
  }
  .toc a.sub.deeper {
    padding-left: 32px;
  }
  .toc a.sub.reading {
    color: var(--accent);
  }
  .switch {
    padding: 0 0 12px;
  }
  .task {
    display: inline-block;
    min-width: 2.2em;
    margin-right: 6px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }
  .column {
    display: flex;
    flex-direction: column;
    gap: 12px;
    min-width: 0;
  }
  .body {
    padding: 18px 28px 24px;
  }
  .title {
    margin-bottom: 10px;
    font-size: 20px;
    letter-spacing: -0.01em;
  }
  .foot {
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    gap: 10px;
  }
  .foot > :last-child {
    justify-self: end;
  }
  .foot .btn.ghost {
    max-width: 100%;
    min-width: 0;
    color: var(--text-2);
  }
  .ell {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .empty p {
    margin: 0 0 4px;
  }
  .small {
    font-size: 12.5px;
  }
  @media (max-width: 900px) {
    .layout {
      grid-template-columns: minmax(0, 1fr);
    }
    .toc {
      position: static;
      max-height: none;
      flex-direction: row;
      flex-wrap: wrap;
      gap: 4px;
    }
    .toc-title {
      flex-basis: 100%;
    }
    .toc a.sub {
      display: none;
    }
  }
</style>
