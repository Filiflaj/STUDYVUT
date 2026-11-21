<script lang="ts">
  import changelogText from "../../CHANGELOG.md?raw";
  import { api, errorMessage } from "../lib/api";
  import { parseChangelog } from "../lib/changelog";
  import WhatsNew from "../lib/components/WhatsNew.svelte";
  import Field from "../lib/components/Field.svelte";
  import Icon from "../lib/components/Icon.svelte";
  import SegmentedControl from "../lib/components/SegmentedControl.svelte";
  import Section from "../lib/components/Section.svelte";
  import Toggle from "../lib/components/Toggle.svelte";
  import { LANGUAGES, type LanguagePref, fill, i18n, languageName, t } from "../lib/i18n/index.svelte";
  import { live } from "../lib/live.svelte";
  import { subjectColor } from "../lib/palette";
  import { theme, type ThemePref } from "../lib/theme.svelte";
  import type { Problem, ProblemCount } from "../lib/types";

  const st = $derived(t().settings);

  let problems = $state<Problem[] | null>(null);
  let showProblems = $state(false);
  let reloading = $state(false);
  let notice = $state<string | null>(null);
  let resetTarget = $state<string>("");
  let confirming = $state(false);
  const changelog = parseChangelog(changelogText);
  let allReleases = $state(false);

  const count = $derived<ProblemCount>(live.overview?.problems ?? { errors: 0, warnings: 0 });
  const folder = $derived(live.app?.content.kind === "folder" ? live.app.content.path : null);

  async function toggleProblems() {
    showProblems = !showProblems;
    if (showProblems) problems = await api.problems().catch(() => []);
  }

  async function reload() {
    reloading = true;
    try {
      await api.reloadContent();
      await live.refresh();
      if (showProblems) problems = await api.problems();
    } finally {
      reloading = false;
    }
  }

  async function exportProgress() {
    notice = null;
    try {
      const path = await api.exportProgress(i18n.lang);
      if (path) notice = fill(st.exported, { path });
    } catch (e) {
      notice = errorMessage(e);
    }
  }

  async function reset() {
    confirming = false;
    try {
      await api.resetProgress(resetTarget || null);
      notice = st.resetDone;
      await live.refresh();
    } catch (e) {
      notice = errorMessage(e);
    }
  }

  const resetWhat = $derived(
    resetTarget ? (live.subject(resetTarget)?.code ?? resetTarget) : st.resetAll.toLowerCase(),
  );
</script>

{#if live.settings}
  {@const s = live.settings}
  <div class="view">
    <div class="page-head"><h1>{st.title}</h1></div>

    <Section title={st.general}>
      <Field label={st.appearance}>
        <SegmentedControl
          label={st.appearance}
          value={s.theme}
          options={[
            { value: "system" as ThemePref, label: st.themeSystem },
            { value: "light" as ThemePref, label: st.themeLight },
            { value: "dark" as ThemePref, label: st.themeDark },
          ]}
          onchange={(v) => live.save((x) => (x.theme = v))}
        />
      </Field>
      <Field label={st.language}>
        <select
          class="select"
          value={s.language}
          onchange={(e) => {
            const v = e.currentTarget.value as LanguagePref;
            live.save((x) => (x.language = v));
          }}
        >
          <option value="system">{fill(st.languageSystem, { name: languageName(live.app?.system_language ?? "cs") })}</option>
          {#each LANGUAGES as l (l.code)}
            <option value={l.code} lang={l.code}>{l.name}</option>
          {/each}
        </select>
      </Field>
    </Section>

    <Section title={st.learning} description={st.subjectsHint}>
      {#each live.subjects as subj (subj.id)}
        {@const shown = !s.hidden_subjects.includes(subj.id)}
        <Field label="{subj.code} · {subj.name}" hint={subj.semester}>
          <span class="dot" style:background={subjectColor(subj.color, theme.effective === "dark")}></span>
          <Toggle
            checked={shown}
            label="{subj.code} {subj.name}"
            onchange={(v) =>
              live.save((x) => {
                x.hidden_subjects = v
                  ? x.hidden_subjects.filter((id) => id !== subj.id)
                  : [...x.hidden_subjects, subj.id];
              })}
          />
        </Field>
      {/each}
    </Section>

    <Section title={st.content}>
      <Field label={st.source}>
        <span class="value" title={folder ?? ""}>{folder ? fill(st.folder, { path: folder }) : st.embedded}</span>
      </Field>
      <Field label={st.problems}>
        {#if count.errors + count.warnings === 0}
          <span class="ok"><Icon name="check" size={14} /> {st.noProblems}</span>
        {:else}
          <button class="btn ghost" class:bad={count.errors > 0} onclick={toggleProblems}>
            <Icon name="warning" size={14} />
            {fill(st.problemsCount, { errors: count.errors, warnings: count.warnings })}
            <Icon name="chevron" size={12} />
          </button>
        {/if}
      </Field>
      {#if showProblems && problems}
        <ul class="problems">
          {#each problems as p, i (i)}
            <li class:error={p.severity === "error"}><code>{p.path}</code> {p.message}</li>
          {/each}
        </ul>
      {/if}
      {#if folder}
        <Field label={st.reload} hint={st.reloadHint}>
          <button class="btn" disabled={reloading} onclick={reload}>
            <Icon name={reloading ? "loader" : "reset"} size={14} />
            {st.reload}
          </button>
        </Field>
      {/if}
    </Section>

    <Section title={st.data}>
      <Field label={st.dataFolder}>
        <span class="value mono" title={live.app?.data_dir ?? ""}>{live.app?.data_dir ?? ""}</span>
        <button class="btn" onclick={() => api.openDataFolder()}><Icon name="folder" size={14} /> {st.openFolder}</button>
      </Field>
      <Field label={st.export} hint={st.exportHint}>
        <button class="btn" onclick={exportProgress}><Icon name="download" size={14} /> {st.exportButton}</button>
      </Field>
      <Field label={st.reset} hint={st.resetHint}>
        <select class="select" bind:value={resetTarget} onchange={() => (confirming = false)}>
          <option value="">{st.resetAll}</option>
          {#each live.subjects as subj (subj.id)}
            <option value={subj.id}>{subj.code}</option>
          {/each}
        </select>
        {#if confirming}
          <button class="btn danger" onclick={reset}>{st.reset}</button>
          <button class="btn ghost" onclick={() => (confirming = false)}>{t().common.cancel}</button>
        {:else}
          <button class="btn" onclick={() => (confirming = true)}>{st.reset}…</button>
        {/if}
      </Field>
      {#if confirming}
        <p class="confirm" role="alert">{fill(st.resetConfirm, { what: resetWhat })}</p>
      {/if}
      {#if notice}<p class="notice" role="status">{notice}</p>{/if}
    </Section>

    <Section title={st.about}>
      <Field label={st.version}>
        <span class="badge">{live.app?.version ?? ""}</span>
      </Field>
      <Field label={st.links}>
        <button class="btn ghost" onclick={() => api.openLink("repository")}>GitHub <Icon name="external" size={12} /></button>
        <button class="btn ghost" onclick={() => api.openLink("releases")}>{t().nav.releases} <Icon name="external" size={12} /></button>
      </Field>
      <p class="about muted">{st.basedOn}</p>
    </Section>

    <Section title={t().whatsNew.title}>
      <div class="whats-new">
        <WhatsNew
          releases={allReleases ? changelog.releases : changelog.releases.slice(0, 3)}
          unreleased={live.app?.debug ? changelog.unreleased : []}
        />
        {#if !allReleases && changelog.releases.length > 3}
          <button class="btn ghost" onclick={() => (allReleases = true)}>{t().whatsNew.showAll}</button>
        {/if}
      </div>
    </Section>
  </div>
{/if}

<style>
  .value {
    max-width: 360px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-2);
  }
  .dot {
    width: 9px;
    height: 9px;
    border-radius: 50%;
  }
  .ok {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    color: var(--good-text);
  }
  .bad {
    color: var(--critical);
  }
  .problems {
    margin: 0;
    padding: 8px 14px 12px 32px;
    max-height: 260px;
    overflow-y: auto;
    font-size: 12.5px;
    color: var(--warning-text);
    -webkit-user-select: text;
    user-select: text;
  }
  .problems li.error {
    color: var(--critical);
  }
  .problems code {
    color: var(--text-2);
  }
  .btn.danger {
    background: var(--critical);
    border-color: var(--critical);
    color: #fff;
  }
  .confirm,
  .notice {
    margin: 0;
    padding: 10px 14px;
    font-size: 13px;
  }
  .confirm {
    background: var(--critical-weak);
    color: var(--critical);
  }
  .notice {
    color: var(--text-2);
    -webkit-user-select: text;
    user-select: text;
  }
  .badge {
    padding: 2px 8px;
    border-radius: 10px;
    background: var(--accent-weak);
    color: var(--accent);
    font-size: 12px;
    font-weight: 600;
  }
  .about {
    margin: 0;
    padding: 12px 14px;
    font-size: 12.5px;
  }
  .whats-new {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
    padding: 12px 14px 14px;
  }
</style>
