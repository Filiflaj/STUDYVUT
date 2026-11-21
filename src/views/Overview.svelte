<script lang="ts">
  import Icon from "../lib/components/Icon.svelte";
  import { fill, plural, t } from "../lib/i18n/index.svelte";
  import { live } from "../lib/live.svelte";
  import { nav } from "../lib/nav.svelte";
  import { subjectColor } from "../lib/palette";
  import { theme } from "../lib/theme.svelte";
  import type { SubjectView } from "../lib/types";

  /** Study years offered: a bachelor's three, and any other year the content uses. */
  const years = $derived(
    [...new Set([1, 2, 3, ...live.subjects.map((s) => s.year).filter((y): y is number => y !== null)])].sort(
      (a, b) => a - b,
    ),
  );
  const programs = $derived(live.overview?.programs ?? []);
  const dark = $derived(theme.effective === "dark");
  const due = $derived(live.visibleSubjects.filter((s) => s.due > 0));
  const dueTotal = $derived(due.reduce((n, s) => n + s.due, 0));
  const program = $derived(live.settings?.program ?? "");
  const year = $derived(live.settings?.year ?? 0);

  /** "ZS 2026/2027" → "Zimní semestr 2026/2027" (as the study system writes it). */
  function semesterName(s: string): string {
    const m = /^(ZS|LS)\s+(.+)$/i.exec(s.trim());
    if (!m) return s;
    return fill(m[1].toUpperCase() === "ZS" ? t().overview.winter : t().overview.summer, { years: m[2] });
  }

  /** The shown subjects grouped by semester, in the backend's order. */
  const semesters = $derived.by(() => {
    const groups: { semester: string; subjects: SubjectView[] }[] = [];
    for (const s of live.visibleSubjects) {
      const g = groups.find((x) => x.semester === s.semester);
      if (g) g.subjects.push(s);
      else groups.push({ semester: s.semester, subjects: [s] });
    }
    return groups;
  });
</script>

<div class="view">
  <div class="page-head">
    <h1>{t().overview.title}</h1>
    <div class="filters">
      <label class="filter">
        <span>{t().overview.program}</span>
        <select
          class="select"
          value={program}
          onchange={(e) => {
            const v = e.currentTarget.value || null;
            live.save((s) => (s.program = v));
          }}
        >
          <option value="">{t().overview.allPrograms}</option>
          {#each programs as p (p.id)}
            <option value={p.id}>{p.code.replace(/^BPC-/, "")} · {p.name}</option>
          {/each}
        </select>
      </label>
      <label class="filter">
        <span>{t().overview.year}</span>
        <select
          class="select"
          value={String(year)}
          onchange={(e) => {
            const v = Number(e.currentTarget.value) || null;
            live.save((s) => (s.year = v));
          }}
        >
          <option value="0">{t().overview.allYears}</option>
          {#each years as y (y)}
            <option value={String(y)}>{fill(t().overview.yearN, { n: y })}</option>
          {/each}
        </select>
      </label>
    </div>
  </div>

  {#if live.firstRun && !live.settings?.program && !live.settings?.year}
    <div class="card welcome">
      <h2>{t().overview.welcome}</h2>
      <p class="muted">{t().overview.welcomeText}</p>
    </div>
  {/if}

  {#if dueTotal > 0}
    <div class="card due">
      <Icon name="reset" size={15} />
      <span class="due-text">{plural(t().overview.due, dueTotal)}</span>
      <div class="due-actions">
        {#each due as s (s.id)}
          <button class="btn" onclick={() => nav.quiz(s.id, { kind: "due" })}>
            <span class="dot" style:background={subjectColor(s.color, dark)}></span>
            {s.code.replace(/^BPC-/, "")}: {s.due}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  {#if live.visibleSubjects.length === 0}
    <div class="card empty">
      {live.subjects.length && (program || year) ? t().overview.noSubjects : t().overview.hiddenAll}
    </div>
  {/if}

  {#each semesters as g (g.semester)}
    <section class="group">
      <h2 class="group-title">{semesterName(g.semester)}</h2>
      <div class="subjects">
        {#each g.subjects as s (s.id)}
          {@const color = subjectColor(s.color, dark)}
          {@const empty = s.chapters.length === 0 && s.questions === 0}
          <article class="card subject" style:--c={color}>
            <header>
              <span class="dot"></span>
              <span class="code">{s.code}</span>
            </header>
            <h3>{s.name}</h3>
            {#if empty}
              <p class="muted small">{t().overview.noContent}</p>
              <p class="muted hint">{fill(t().overview.noContentHint, { id: s.id })}</p>
            {:else}
              <p class="muted small">
                {plural(t().overview.chapters, s.chapters.length)} · {plural(t().overview.questions, s.questions)}
              </p>
              <div class="actions">
                {#if s.chapters.length}
                  <button class="btn" onclick={() => nav.study(s.id)}><Icon name="book" size={13} /> {t().overview.study}</button>
                {/if}
                {#if s.questions}
                  <button class="btn" onclick={() => nav.quiz(s.id)}><Icon name="quiz" size={13} /> {t().overview.quizzes}</button>
                {/if}
                {#if s.due > 0}
                  <button class="btn primary" onclick={() => nav.quiz(s.id, { kind: "due" })}>
                    {plural(t().overview.review, s.due)}
                  </button>
                {/if}
              </div>
            {/if}
          </article>
        {/each}
      </div>
    </section>
  {/each}
</div>

<style>
  .filters {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }
  .filter {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12.5px;
    font-weight: 500;
    color: var(--text-muted);
  }
  .filter .select {
    max-width: 280px;
    font-size: 13.5px;
    color: var(--text);
  }
  .welcome {
    padding: 16px 18px;
  }
  .welcome h2 {
    font-size: 16px;
    margin-bottom: 4px;
  }
  .welcome p {
    margin: 0;
  }
  .due {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 8px 12px;
    padding: 10px 14px;
    background: var(--accent-weak);
    border-color: transparent;
    color: var(--accent);
  }
  .due-text {
    font-weight: 600;
  }
  .due-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-left: auto;
  }
  .due-actions .btn {
    height: 28px;
    padding: 0 10px;
    font-size: 13px;
    color: var(--text);
  }
  .group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .group-title {
    padding: 0 4px;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-2);
  }
  .subjects {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 12px;
  }
  .subject {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 14px 16px;
    border-top: 3px solid var(--c);
  }
  .subject header {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .dot {
    flex: none;
    width: 9px;
    height: 9px;
    border-radius: 50%;
    background: var(--c);
  }
  .code {
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: var(--text-2);
  }
  .subject h3 {
    font-size: 15.5px;
    font-weight: 600;
  }
  .subject p {
    margin: 0;
  }
  .small {
    font-size: 12.5px;
  }
  .hint {
    font-size: 12px;
  }
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: auto;
    padding-top: 6px;
  }
  .actions .btn {
    height: 28px;
    padding: 0 10px;
    font-size: 13px;
  }
</style>
