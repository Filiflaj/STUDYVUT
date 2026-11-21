<script lang="ts">
  import { api } from "../lib/api";
  import ActivityChart from "../lib/components/ActivityChart.svelte";
  import Icon from "../lib/components/Icon.svelte";
  import ProgressBar from "../lib/components/ProgressBar.svelte";
  import { fmtDateTime, fmtToday, percent } from "../lib/format";
  import { fill, plural, t } from "../lib/i18n/index.svelte";
  import { live } from "../lib/live.svelte";
  import { nav } from "../lib/nav.svelte";
  import { subjectColor } from "../lib/palette";
  import { setName } from "../lib/sets";
  import { theme } from "../lib/theme.svelte";
  import type { DayActivity, SubjectView } from "../lib/types";

  const o = $derived(live.overview);
  const goal = $derived(live.settings?.daily_goal ?? 20);
  const dark = $derived(theme.effective === "dark");
  const dueTotal = $derived(live.visibleSubjects.reduce((n, s) => n + s.progress.due, 0));
  const accuracy = $derived(o ? percent(o.correct_today, o.answered_today) : null);

  /** Subjects grouped by semester, in the backend's order. */
  const semesters = $derived.by(() => {
    const groups: { semester: string; subjects: SubjectView[] }[] = [];
    for (const s of live.visibleSubjects) {
      const g = groups.find((x) => x.semester === s.semester);
      if (g) g.subjects.push(s);
      else groups.push({ semester: s.semester, subjects: [s] });
    }
    return groups;
  });

  let activity = $state<DayActivity[]>([]);
  $effect(() => {
    void live.overview;
    api.activity(30).then((a) => (activity = a), console.error);
  });

  /** The subject with the most questions due, for the tile's button. */
  const mostDue = $derived([...live.visibleSubjects].sort((a, b) => b.progress.due - a.progress.due)[0]);
</script>

<div class="view">
  <div class="page-head">
    <h1>{t().overview.title}</h1>
    <span class="muted today">{fmtToday()}</span>
  </div>

  {#if o && o.recent.length === 0 && o.answered_today === 0}
    <div class="card welcome">
      <h2>{t().overview.welcome}</h2>
      <p class="muted">{t().overview.welcomeText}</p>
    </div>
  {/if}

  {#if o}
    <div class="tiles">
      <div class="card tile">
        <div class="tile-head"><Icon name="target" size={14} /> {t().overview.today}</div>
        <div class="big tabular">{o.answered_today}<span class="of">/ {goal}</span></div>
        <div class="goal-bar" aria-hidden="true">
          <div style:width="{Math.min(100, (o.answered_today / goal) * 100)}%"></div>
        </div>
        <div class="sub muted">
          {o.answered_today >= goal ? t().overview.goalDone : fill(t().overview.goal, { n: o.answered_today, goal })}
        </div>
      </div>
      <div class="card tile">
        <div class="tile-head"><Icon name="check" size={14} /> {t().overview.accuracy}</div>
        <div class="big tabular">{accuracy === null ? "–" : `${accuracy} %`}</div>
        <div class="sub muted">
          {accuracy === null ? t().overview.accuracyNone : `${o.correct_today} / ${o.answered_today}`}
        </div>
      </div>
      <div class="card tile">
        <div class="tile-head"><Icon name="flame" size={14} /> {t().overview.streak}</div>
        <div class="big tabular">{plural(t().overview.streakDays, o.streak)}</div>
        <div class="sub muted">{t().overview.streakHint}</div>
      </div>
      <div class="card tile">
        <div class="tile-head"><Icon name="reset" size={14} /> {t().overview.due}</div>
        <div class="big tabular">{dueTotal}</div>
        {#if dueTotal > 0 && mostDue}
          <button class="btn small" onclick={() => nav.quiz(mostDue.id, { kind: "due" })}>
            <Icon name="play" size={12} />
            {mostDue.code}: {plural(t().overview.review, mostDue.progress.due)}
          </button>
        {:else}
          <div class="sub muted">{t().overview.dueHint}</div>
        {/if}
      </div>
    </div>

    <section class="card chart-card">
      <div class="card-head">
        <h2>{t().overview.activity}</h2>
        <span class="muted small">{t().overview.activityHint}</span>
      </div>
      <ActivityChart days={activity} {goal} />
    </section>

    {#if live.visibleSubjects.length === 0}
      <div class="card empty">{t().overview.hiddenAll}</div>
    {/if}

    {#each semesters as g (g.semester)}
      <section class="group">
        <h2 class="group-title">{t().overview.subjects} · {g.semester}</h2>
        <div class="subjects">
          {#each g.subjects as s (s.id)}
            {@const color = subjectColor(s.color, dark)}
            {@const empty = s.chapters.length === 0 && s.progress.questions === 0}
            <article class="card subject">
              <header>
                <span class="dot" style:background={color}></span>
                <span class="code">{s.code}</span>
                <span class="name">{s.name}</span>
              </header>
              {#if empty}
                <p class="muted small">{t().overview.noContent}</p>
                <p class="muted hint">{fill(t().overview.noContentHint, { id: s.id })}</p>
              {:else}
                <ProgressBar progress={s.progress} {color} />
                <div class="stats muted small">
                  {fill(t().progress.summary, {
                    known: s.progress.correct,
                    mistakes: s.progress.wrong,
                    unseen: s.progress.questions - s.progress.correct - s.progress.wrong,
                  })}
                </div>
                <div class="actions">
                  {#if s.chapters.length}
                    <button class="btn" onclick={() => nav.study(s.id)}><Icon name="book" size={13} /> {t().overview.study}</button>
                  {/if}
                  {#if s.progress.questions}
                    <button class="btn" onclick={() => nav.quiz(s.id)}><Icon name="quiz" size={13} /> {t().overview.quizzes}</button>
                  {/if}
                  {#if s.progress.due > 0}
                    <button class="btn primary" onclick={() => nav.quiz(s.id, { kind: "due" })}>
                      {plural(t().overview.review, s.progress.due)}
                    </button>
                  {/if}
                </div>
              {/if}
            </article>
          {/each}
        </div>
      </section>
    {/each}

    <section class="card recent">
      <div class="card-head"><h2>{t().overview.recent}</h2></div>
      {#if o.recent.length === 0}
        <p class="muted small pad">{t().overview.noRecent}</p>
      {:else}
        <ul>
          {#each o.recent as r (r.id)}
            {@const s = live.subject(r.subject)}
            {@const pct = percent(r.correct, r.total)}
            <li>
              <span class="dot" style:background={subjectColor(s?.color, dark)}></span>
              <span class="what">
                <span class="set">{setName(s, r.set_key)}</span>
                <span class="muted small">{s?.code ?? r.subject} · {fmtDateTime(r.started)}</span>
              </span>
              <span class="score tabular">{r.correct} / {r.total}</span>
              <span class="pct tabular muted">{pct ?? 0} %</span>
            </li>
          {/each}
        </ul>
      {/if}
    </section>
  {/if}
</div>

<style>
  .today {
    font-size: 13px;
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
  .tiles {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 12px;
  }
  .tile {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 14px 16px;
    min-height: 118px;
  }
  .tile-head {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12.5px;
    font-weight: 500;
    color: var(--text-2);
  }
  .tile-head :global(svg) {
    color: var(--accent);
  }
  .big {
    font-size: 28px;
    font-weight: 600;
    letter-spacing: -0.01em;
    line-height: 1.1;
  }
  .of {
    margin-left: 4px;
    font-size: 15px;
    font-weight: 500;
    color: var(--text-muted);
  }
  .sub {
    font-size: 12.5px;
  }
  .goal-bar {
    height: 6px;
    border-radius: 99px;
    background: var(--grid);
    overflow: hidden;
  }
  .goal-bar > div {
    height: 100%;
    border-radius: 99px;
    background: var(--accent);
  }
  .btn.small {
    align-self: flex-start;
    height: 26px;
    padding: 0 10px;
    font-size: 12.5px;
  }
  .chart-card {
    padding: 14px 16px;
  }
  .card-head {
    display: flex;
    align-items: baseline;
    gap: 10px;
    flex-wrap: wrap;
    margin-bottom: 8px;
  }
  .card-head h2,
  .group-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-2);
  }
  .group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .group-title {
    padding: 0 4px;
  }
  .subjects {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(290px, 1fr));
    gap: 12px;
  }
  .subject {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 14px 16px;
  }
  .subject header {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }
  .dot {
    flex: none;
    width: 9px;
    height: 9px;
    border-radius: 50%;
  }
  .code {
    font-weight: 600;
  }
  .name {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-2);
  }
  .subject p {
    margin: 0;
  }
  .hint {
    font-size: 12px;
  }
  .small {
    font-size: 12.5px;
  }
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 2px;
  }
  .actions .btn {
    height: 28px;
    padding: 0 10px;
    font-size: 13px;
  }
  .recent {
    padding: 14px 16px 6px;
  }
  .recent ul {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .recent li {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 0;
    border-top: 1px solid var(--border);
  }
  .recent .what {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }
  .recent .set {
    font-weight: 500;
  }
  .recent .score {
    font-weight: 600;
  }
  .recent .pct {
    width: 48px;
    text-align: right;
  }
  .pad {
    margin: 0 0 8px;
  }
</style>
