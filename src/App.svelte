<script lang="ts">
  import { onMount } from "svelte";
  import changelogText from "../CHANGELOG.md?raw";
  import { api } from "./lib/api";
  import { compareVersions, newSince, parseChangelog, type Release } from "./lib/changelog";
  import Icon from "./lib/components/Icon.svelte";
  import WhatsNew from "./lib/components/WhatsNew.svelte";
  import PrefsSwitcher from "./lib/components/PrefsSwitcher.svelte";
  import { percent } from "./lib/format";
  import { fill, t } from "./lib/i18n/index.svelte";
  import { live } from "./lib/live.svelte";
  import { nav, type Tab } from "./lib/nav.svelte";
  import { subjectColor } from "./lib/palette";
  import { quiz } from "./lib/quiz.svelte";
  import { theme } from "./lib/theme.svelte";
  import Data from "./views/Data.svelte";
  import Manual from "./views/Manual.svelte";
  import Overview from "./views/Overview.svelte";
  import Quizzes from "./views/Quizzes.svelte";
  import Settings from "./views/Settings.svelte";
  import Study from "./views/Study.svelte";

  const TABS: { key: Tab; icon: string }[] = [
    { key: "overview", icon: "gauge" },
    { key: "study", icon: "book" },
    { key: "quiz", icon: "quiz" },
    { key: "data", icon: "table" },
    { key: "settings", icon: "sliders" },
    { key: "manual", icon: "help" },
  ];

  let failed = $state<string | null>(null);

  /** The sidebar folded to its icons; remembered. */
  const COLLAPSED_KEY = "studyvut.sidebar.collapsed";
  let collapsed = $state(readCollapsed());

  function readCollapsed(): boolean {
    try {
      return localStorage.getItem(COLLAPSED_KEY) === "1";
    } catch {
      return false;
    }
  }

  function toggleSidebar() {
    collapsed = !collapsed;
    try {
      localStorage.setItem(COLLAPSED_KEY, collapsed ? "1" : "0");
    } catch {
      // Private mode: the sidebar opens expanded next time.
    }
  }

  onMount(() => {
    live
      .init()
      .then(whatsNewOnce)
      .catch((e) => (failed = String(e)));
  });

  /** "Co je nového" once after an update: the releases since the version last seen. */
  let fresh = $state<Release[]>([]);

  function whatsNewOnce() {
    const current = live.app?.version;
    const seen = live.settings?.last_seen_version ?? null;
    if (!current) return;
    fresh = newSince(parseChangelog(changelogText), seen, current);
    // A fresh install or an older build shows nothing, but remembers the version.
    if (fresh.length === 0 && (seen === null || compareVersions(seen, current) !== 0)) seenNow();
  }

  function seenNow() {
    const current = live.app?.version;
    fresh = [];
    if (current) void live.save((s) => (s.last_seen_version = current));
  }

  const dueTotal = $derived(live.visibleSubjects.reduce((n, s) => n + s.progress.due, 0));

  /** A subject in the sidebar opens it on the Study or Quiz page, whichever is open. */
  function openSubject(id: string) {
    if (nav.tab === "quiz") nav.quiz(id);
    else nav.study(id);
  }
</script>

<!-- The app's symbol, also its icon (src-tauri/icons/icon.svg). -->
{#snippet symbol(size: number)}
  <svg width={size} height={size} viewBox="100 100 824 824" aria-hidden="true">
    <rect x="100" y="100" width="824" height="824" rx="185" fill="var(--accent)" />
    <path d="M512 352c-62-44-150-62-264-55v388c114-7 202 11 264 55z" fill="#fff" />
    <path d="M512 352c62-44 150-62 264-55v388c-114-7-202 11-264 55z" fill="#dce5ff" />
    <path
      d="M574 548l52 52 104-122"
      fill="none"
      stroke="var(--accent)"
      stroke-width="40"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </svg>
{/snippet}

<!-- Arrow markers of the figures (docs/content.md), defined once. -->
<svg width="0" height="0" style="position: absolute" aria-hidden="true">
  <defs>
    <marker id="m-f" viewBox="0 0 10 10" refX="9" refY="5" markerUnits="userSpaceOnUse" markerWidth="13" markerHeight="13" orient="auto"><path d="M0,1 L10,5 L0,9 z" class="mkA" /></marker>
    <marker id="m-fl" viewBox="0 0 10 10" refX="9" refY="5" markerUnits="userSpaceOnUse" markerWidth="11" markerHeight="11" orient="auto"><path d="M0,1 L10,5 L0,9 z" class="mkF" /></marker>
    <marker id="m-v" viewBox="0 0 10 10" refX="9" refY="5" markerUnits="userSpaceOnUse" markerWidth="11" markerHeight="11" orient="auto"><path d="M0,1 L10,5 L0,9 z" class="mkI" /></marker>
    <marker id="m-badl" viewBox="0 0 10 10" refX="9" refY="5" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto"><path d="M0,1 L10,5 L0,9 z" class="mkB" /></marker>
  </defs>
</svg>

<div class="app" class:collapsed>
  <aside class="sidebar">
    <button class="brand" type="button" title="STUDYVUT" onclick={() => nav.go("overview")}>
      {@render symbol(collapsed ? 26 : 28)}
      {#if !collapsed}<span class="wordmark">STUDY<span>VUT</span></span>{/if}
    </button>
    <nav>
      {#each TABS as tab_ (tab_.key)}
        <button
          data-tab={tab_.key}
          class:active={nav.tab === tab_.key}
          aria-current={nav.tab === tab_.key ? "page" : undefined}
          title={collapsed ? t().nav[tab_.key] : undefined}
          aria-label={collapsed ? t().nav[tab_.key] : undefined}
          onclick={() => nav.go(tab_.key)}
        >
          <Icon name={tab_.icon} size={16} />
          {#if !collapsed}<span>{t().nav[tab_.key]}</span>{/if}
          {#if tab_.key === "quiz" && quiz.active && !quiz.done}
            <span class="live-dot" aria-hidden="true"></span>
          {:else if tab_.key === "quiz" && dueTotal > 0}
            <span class="count" title={t().overview.due}>{dueTotal}</span>
          {/if}
        </button>
      {/each}
    </nav>
    {#if !collapsed}
      <div class="bottom">
        <PrefsSwitcher />
      </div>
      {#if live.visibleSubjects.length}
        <div class="status">
          <div class="status-title">{t().nav.subjects}</div>
          {#each live.visibleSubjects as s (s.id)}
            {@const pct = percent(s.progress.correct, s.progress.questions)}
            <button
              class="subject"
              class:current={nav.subject === s.id && (nav.tab === "study" || nav.tab === "quiz")}
              title="{s.code} {s.name}"
              onclick={() => openSubject(s.id)}
            >
              <span class="dot" style:background={subjectColor(s.color, theme.effective === "dark")}></span>
              <span class="label">{s.code.replace(/^BPC-/, "")}</span>
              <span class="name">{s.name}</span>
              {#if pct !== null}<span class="pct tabular">{pct} %</span>{/if}
            </button>
          {/each}
        </div>
      {/if}
      <div class="about">
        <div class="links">
          <a
            href="https://github.com/Filiflaj/STUDYVUT"
            onclick={(e) => {
              e.preventDefault();
              api.openLink("repository");
            }}>{t().nav.repository} <Icon name="external" size={11} /></a
          >
          <a
            href="https://github.com/Filiflaj/STUDYVUT/releases"
            onclick={(e) => {
              e.preventDefault();
              api.openLink("releases");
            }}>{t().nav.releases} <Icon name="external" size={11} /></a
          >
        </div>
        {#if live.app}<div class="version">{fill(t().nav.version, { version: live.app.version })}</div>{/if}
      </div>
    {/if}
    <button
      class="fold"
      type="button"
      title={collapsed ? t().nav.expand : undefined}
      aria-label={collapsed ? t().nav.expand : undefined}
      aria-expanded={!collapsed}
      onclick={toggleSidebar}
    >
      <Icon name={collapsed ? "chevron-right" : "chevron-left"} size={14} />
      {#if !collapsed}<span>{t().nav.collapse}</span>{/if}
    </button>
  </aside>

  <main>
    {#if live.saveError}
      <div class="save-error" role="alert">
        <Icon name="warning" size={14} />
        <span>{fill(t().common.saveFailed, { error: live.saveError })}</span>
        <button class="close" type="button" aria-label={t().common.close} onclick={() => (live.saveError = null)}>
          <Icon name="close" size={14} />
        </button>
      </div>
    {/if}
    {#if failed}
      <div class="center muted">{fill(t().common.backendError, { error: failed })}</div>
    {:else if !live.ready}
      <div class="center muted">{t().common.loading}</div>
    {:else if nav.tab === "overview"}
      <Overview />
    {:else if nav.tab === "study"}
      <Study />
    {:else if nav.tab === "quiz"}
      <Quizzes />
    {:else if nav.tab === "data"}
      <Data />
    {:else if nav.tab === "manual"}
      <Manual />
    {:else}
      <Settings />
    {/if}
  </main>
</div>

{#if fresh.length}
  <div class="modal-backdrop">
    <div class="card modal" role="dialog" aria-modal="true" aria-labelledby="whats-new-title">
      <h2 id="whats-new-title">{t().whatsNew.title}</h2>
      <div class="modal-body"><WhatsNew releases={fresh} /></div>
      <div class="modal-actions">
        <button class="btn primary" onclick={seenNow}>{t().common.close}</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .app {
    display: grid;
    grid-template-columns: 216px 1fr;
    height: 100%;
    grid-template-rows: minmax(0, 1fr);
  }
  .sidebar {
    display: flex;
    flex-direction: column;
    gap: 14px;
    min-height: 0;
    overflow-y: auto;
    padding: 16px 12px 12px;
    background: var(--sidebar);
    border-right: 1px solid var(--border);
  }
  .app.collapsed {
    grid-template-columns: 60px 1fr;
  }
  .collapsed .sidebar {
    padding: 18px 10px 14px;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 4px 8px;
    margin: -2px 0;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: inherit;
    cursor: pointer;
  }
  .collapsed .brand {
    justify-content: center;
    padding: 4px 0;
  }
  .brand:hover {
    background: var(--hover);
  }
  .brand :global(svg) {
    flex: none;
    display: block;
  }
  .wordmark {
    font-size: 16px;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: var(--text);
  }
  .wordmark span {
    color: var(--accent);
  }
  nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  nav button {
    display: flex;
    align-items: center;
    gap: 10px;
    height: 32px;
    padding: 0 10px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-2);
    font-weight: 500;
    text-align: left;
    cursor: pointer;
  }
  nav button:hover {
    background: var(--hover);
    color: var(--text);
  }
  nav button.active {
    background: var(--surface);
    color: var(--text);
    box-shadow: var(--shadow), inset 0 0 0 1px var(--border);
  }
  nav button.active :global(svg) {
    color: var(--accent);
  }
  .collapsed nav button {
    position: relative;
    justify-content: center;
    padding: 0;
  }
  nav span:not(.count):not(.live-dot) {
    flex: 1;
  }
  .count {
    min-width: 18px;
    height: 18px;
    padding: 0 5px;
    border-radius: 9px;
    background: var(--accent);
    color: var(--on-accent);
    font-size: 11px;
    font-weight: 700;
    line-height: 18px;
    text-align: center;
  }
  .collapsed .count {
    position: absolute;
    top: 1px;
    right: 1px;
    min-width: 15px;
    height: 15px;
    padding: 0 3px;
    font-size: 9.5px;
    line-height: 15px;
  }
  .live-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-weak);
  }
  .collapsed .live-dot {
    position: absolute;
    top: 5px;
    right: 6px;
  }
  .fold {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
    height: 28px;
    margin: -4px 0 0;
    padding: 0 8px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    font-size: 12.5px;
    cursor: pointer;
  }
  .fold:hover {
    background: var(--hover);
    color: var(--text);
  }
  .collapsed .fold {
    justify-content: center;
    margin-top: auto;
    padding: 0;
  }
  .bottom {
    margin-top: auto;
    padding: 0 2px;
  }
  .status {
    padding: 10px 0 0;
    border-top: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .status-title {
    padding: 0 10px 4px;
    font-size: 11.5px;
    font-weight: 500;
    color: var(--text-muted);
  }
  .subject {
    display: flex;
    align-items: center;
    gap: 7px;
    min-width: 0;
    height: 24px;
    padding: 0 10px;
    border: none;
    border-radius: 6px;
    background: transparent;
    font-size: 12.5px;
    font-weight: 500;
    color: var(--text-2);
    white-space: nowrap;
    text-align: left;
    cursor: pointer;
  }
  .subject:hover {
    background: var(--hover);
    color: var(--text);
  }
  .subject.current {
    color: var(--text);
    background: var(--surface-2);
    box-shadow: inset 2px 0 0 var(--accent);
  }
  .subject .label {
    flex: none;
    min-width: 34px;
  }
  .subject .name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--text-muted);
    font-weight: 400;
  }
  .subject .pct {
    flex: none;
    color: var(--text-muted);
    font-size: 11.5px;
  }
  .subject .dot {
    flex-shrink: 0;
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }
  .about {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 10px 10px 0;
    border-top: 1px solid var(--border);
    font-size: 12px;
  }
  .links {
    display: flex;
    flex-wrap: wrap;
    gap: 4px 12px;
  }
  .links a {
    display: inline-flex;
    align-items: center;
    gap: 3px;
    color: var(--text-2);
    text-decoration: none;
  }
  .links a:hover {
    color: var(--accent);
    text-decoration: underline;
  }
  .version {
    color: var(--text-muted);
    font-size: 11.5px;
  }
  main {
    overflow-y: auto;
    min-width: 0;
  }
  .save-error {
    position: sticky;
    top: 0;
    z-index: 5;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    background: var(--critical-weak);
    color: var(--critical);
    font-size: 13px;
  }
  .save-error span {
    flex: 1;
  }
  .save-error .close {
    display: grid;
    place-items: center;
    width: 24px;
    height: 24px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: inherit;
    cursor: pointer;
  }
  main :global(.view) {
    display: flex;
    flex-direction: column;
    gap: 16px;
    max-width: 1100px;
    margin: 0 auto;
    padding: 22px 26px 32px;
  }
  main :global(.page-head) {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
    min-height: 32px;
  }
  main :global(.page-head h1) {
    font-size: 22px;
    letter-spacing: -0.01em;
  }
  main :global(.empty) {
    padding: 28px 20px;
    text-align: center;
    color: var(--text-muted);
  }
  .center {
    display: grid;
    place-items: center;
    height: 100%;
  }
  .modal-backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
    display: grid;
    place-items: center;
    padding: 32px;
    background: rgba(10, 10, 16, 0.45);
  }
  .modal {
    display: flex;
    flex-direction: column;
    gap: 12px;
    width: min(560px, 100%);
    max-height: 80vh;
    padding: 18px 20px;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.25);
  }
  .modal h2 {
    font-size: 17px;
  }
  .modal-body {
    overflow-y: auto;
  }
  .modal-actions {
    display: flex;
    justify-content: flex-end;
  }
</style>
