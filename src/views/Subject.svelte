<script lang="ts">
  /**
   * A subject's own page: from here the user picks what to do in it (the
   * user, 2026-11-21): study text, quizzes, how the subject is assessed, the
   * submitted work, and today's review.
   */
  import { api } from "../lib/api";
  import Icon from "../lib/components/Icon.svelte";
  import { fill, plural, t } from "../lib/i18n/index.svelte";
  import { live } from "../lib/live.svelte";
  import { nav } from "../lib/nav.svelte";
  import { subjectColor } from "../lib/palette";
  import { semesterName, studyChapters } from "../lib/sets";
  import { theme } from "../lib/theme.svelte";
  import type { SubmissionRow } from "../lib/types";

  const subject = $derived(live.subject(nav.subject) ?? live.defaultSubject);
  const color = $derived(subjectColor(subject?.color, theme.effective === "dark"));
  const info = $derived(subject?.chapters.find((c) => c.id === subject?.info));
  const chapters = $derived(studyChapters(subject));
  const program = $derived(
    live.overview?.programs.filter((p) => subject?.programs.includes(p.id)).map((p) => p.code.replace(/^BPC-/, "")) ?? [],
  );

  const meta = $derived(
    subject
      ? [semesterName(subject.semester), subject.year ? fill(t().subject.year, { n: subject.year }) : "", program.join(", ")]
          .filter(Boolean)
          .join(" · ")
      : "",
  );

  let works = $state<SubmissionRow[]>([]);
  $effect(() => {
    const id = subject?.id;
    works = [];
    if (id && subject.submissions > 0) api.submissions(id).then((w) => (works = w), console.error);
  });
</script>

{#if subject}
  <div class="view" style:--c={color}>
    <button class="back" onclick={() => nav.go("overview")}>
      <Icon name="chevron-left" size={14} />
      {t().nav.overview}
    </button>
    <header class="head">
      <div class="eyebrow">
        <span class="dot"></span>
        <span class="code">{subject.code}</span>
        <span class="muted">· {meta}</span>
      </div>
      <h1>{subject.name}</h1>
      {#if subject.description}<p class="muted">{subject.description}</p>{/if}
    </header>

    {#if subject.due > 0}
      <div class="card due">
        <Icon name="reset" size={15} />
        <span>{plural(t().subject.due, subject.due)}</span>
        <button class="btn primary" onclick={() => subject && nav.quiz(subject.id, { kind: "due" })}>
          <Icon name="play" size={12} />
          {t().subject.review}
        </button>
      </div>
    {/if}

    {#if chapters.length === 0 && subject.questions === 0 && subject.submissions === 0}
      <div class="card empty">{fill(t().subject.empty, { id: subject.id })}</div>
    {/if}

    <div class="tiles">
      <button class="card tile" disabled={chapters.length === 0} onclick={() => subject && nav.study(subject.id)}>
        <span class="icon"><Icon name="book" size={20} /></span>
        <span class="title">{t().subject.study}</span>
        <span class="hint">
          {chapters.length ? plural(t().subject.studyHint, chapters.length) : t().subject.none}
        </span>
      </button>

      <button class="card tile" disabled={subject.questions === 0} onclick={() => subject && nav.quiz(subject.id)}>
        <span class="icon"><Icon name="quiz" size={20} /></span>
        <span class="title">{t().subject.quizzes}</span>
        <span class="hint">
          {#if subject.questions}
            {fill(t().subject.quizzesHint, {
              questions: plural(t().overview.questions, subject.questions),
              sets: plural(t().subject.sets, subject.sets.filter((s) => s.kind === "custom" || s.kind === "quiz").length),
            })}
          {:else}
            {t().subject.none}
          {/if}
        </span>
      </button>

      <button class="card tile" disabled={!info} onclick={() => subject && info && nav.info(subject.id)}>
        <span class="icon"><Icon name="info" size={20} /></span>
        <span class="title">{t().subject.info}</span>
        <span class="hint">{info ? t().subject.infoHint : t().subject.none}</span>
      </button>

      <button
        class="card tile"
        disabled={works.length === 0}
        onclick={() => subject && works[0] && nav.work(subject.id, works[0].id)}
      >
        <span class="icon"><Icon name="file-text" size={20} /></span>
        <span class="title">{t().subject.works}</span>
        <span class="hint">{subject.submissions ? plural(t().subject.worksHint, subject.submissions) : t().subject.none}</span>
      </button>
    </div>
  </div>
{/if}

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
  .head {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .head h1 {
    font-size: 24px;
    letter-spacing: -0.01em;
  }
  .head p {
    margin: 2px 0 0;
    max-width: 70ch;
  }
  .eyebrow {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12.5px;
  }
  .dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--c);
  }
  .code {
    font-weight: 600;
    letter-spacing: 0.04em;
    color: var(--text-2);
  }
  .due {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    background: var(--accent-weak);
    border-color: transparent;
    color: var(--accent);
    font-weight: 600;
  }
  .due .btn {
    margin-left: auto;
  }
  .tiles {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 12px;
  }
  .tile {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
    min-height: 132px;
    padding: 16px 18px;
    text-align: left;
    color: var(--text);
    cursor: pointer;
    transition:
      border-color 0.15s,
      box-shadow 0.15s;
  }
  .tile:hover:not(:disabled) {
    border-color: var(--c);
    box-shadow: 0 4px 16px rgba(20, 20, 30, 0.08);
  }
  .tile:disabled {
    cursor: default;
    opacity: 0.55;
  }
  .icon {
    display: grid;
    place-items: center;
    width: 38px;
    height: 38px;
    margin-bottom: 6px;
    border-radius: 10px;
    background: color-mix(in srgb, var(--c) 14%, transparent);
    color: var(--c);
  }
  .title {
    font-size: 15.5px;
    font-weight: 600;
  }
  .hint {
    font-size: 12.5px;
    color: var(--text-muted);
  }
  @media (prefers-reduced-motion: reduce) {
    .tile {
      transition: none;
    }
  }
</style>
