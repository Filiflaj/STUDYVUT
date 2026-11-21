<script lang="ts">
  import { tick } from "svelte";
  import Prose from "../lib/components/Prose.svelte";
  import { i18n, t } from "../lib/i18n/index.svelte";
  import { render } from "../lib/markdown";

  /** The manual, bundled from docs/manual/<lang>.md at build time. */
  const SOURCES = import.meta.glob<string>("../../docs/manual/*.md", { query: "?raw", import: "default", eager: true });

  const source = $derived(SOURCES[`../../docs/manual/${i18n.lang}.md`] ?? SOURCES["../../docs/manual/cs.md"] ?? "");
  const title = $derived(/^# (.+)$/m.exec(source)?.[1] ?? t().nav.manual);
  const manual = $derived(render(source.replace(/^# .+\n+/m, "")));

  let article = $state<HTMLElement>();
  let active = $state("");

  function spy() {
    const top = (article?.closest("main")?.getBoundingClientRect().top ?? 0) + 80;
    let current = manual.sections[0]?.id ?? "";
    for (const s of manual.sections) {
      const h = article?.querySelector<HTMLElement>(`#${CSS.escape(s.id)}`);
      if (h && h.getBoundingClientRect().top <= top) current = s.id;
    }
    active = current;
  }

  $effect(() => {
    void manual;
    const main = article?.closest("main");
    tick().then(spy);
    main?.addEventListener("scroll", spy, { passive: true });
    return () => main?.removeEventListener("scroll", spy);
  });
</script>

<div class="view">
  <div class="page-head"><h1>{title}</h1></div>
  <div class="layout">
    <nav class="toc" aria-label={t().manual.contents}>
      <div class="toc-title">{t().manual.contents}</div>
      {#each manual.sections as s (s.id)}
        <a
          href="#{s.id}"
          class:sub={s.level > 2}
          class:active={active === s.id}
          onclick={(e) => {
            e.preventDefault();
            article?.querySelector(`#${CSS.escape(s.id)}`)?.scrollIntoView({ behavior: "smooth", block: "start" });
          }}>{s.title}</a
        >
      {/each}
    </nav>
    <div class="card body">
      <Prose html={manual.html} tag="article" bind:article />
    </div>
  </div>
</div>

<style>
  .layout {
    display: grid;
    grid-template-columns: 200px minmax(0, 1fr);
    gap: 20px;
    align-items: start;
  }
  .toc {
    position: sticky;
    top: 18px;
    display: flex;
    flex-direction: column;
    gap: 1px;
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
  .toc a.sub {
    padding-left: 22px;
    font-size: 12.5px;
  }
  .toc a:hover {
    background: var(--hover);
    color: var(--text);
  }
  .toc a.active {
    color: var(--text);
    background: var(--surface-2);
    box-shadow: inset 2px 0 0 var(--accent);
  }
  .body {
    padding: 8px 28px 24px;
  }
  @media (max-width: 900px) {
    .layout {
      grid-template-columns: minmax(0, 1fr);
    }
    .toc {
      position: static;
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
