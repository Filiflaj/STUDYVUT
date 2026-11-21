<script lang="ts">
  import Icon from "../lib/components/Icon.svelte";
  import { fill, plural, t } from "../lib/i18n/index.svelte";
  import { live } from "../lib/live.svelte";
  import { nav } from "../lib/nav.svelte";
  import { subjectColor } from "../lib/palette";
  import { semesterName, studyChapters } from "../lib/sets";
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
  const program = $derived(live.settings?.program ?? "");
  const year = $derived(live.settings?.year ?? 0);

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
          {@const empty = studyChapters(s).length === 0 && s.questions === 0}
          <button class="card subject" style:--c={color} onclick={() => nav.open(s.id)}>
            <header>
              <span class="dot"></span>
              <span class="code">{s.code}</span>
              {#if s.due > 0}<span class="badge">{plural(t().overview.dueBadge, s.due)}</span>{/if}
            </header>
            <h3>{s.name}</h3>
            <p class="muted small">
              {#if empty}
                {t().overview.noContent}
              {:else}
                {plural(t().overview.chapters, studyChapters(s).length)} · {plural(t().overview.questions, s.questions)}
              {/if}
            </p>
            <Icon name="chevron-right" size={16} />
          </button>
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
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 6px;
    min-height: 112px;
    padding: 14px 40px 14px 16px;
    border-top: 3px solid var(--c);
    text-align: left;
    color: var(--text);
    cursor: pointer;
    transition:
      border-color 0.15s,
      box-shadow 0.15s;
  }
  .subject:hover {
    border-color: var(--c);
    box-shadow: 0 4px 16px rgba(20, 20, 30, 0.08);
  }
  .subject > :global(svg) {
    position: absolute;
    right: 14px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-muted);
  }
  .subject:hover > :global(svg) {
    color: var(--c);
  }
  .badge {
    margin-left: auto;
    padding: 1px 8px;
    border-radius: 99px;
    background: var(--accent-weak);
    color: var(--accent);
    font-size: 11.5px;
    font-weight: 600;
  }
  @media (prefers-reduced-motion: reduce) {
    .subject {
      transition: none;
    }
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
</style>
