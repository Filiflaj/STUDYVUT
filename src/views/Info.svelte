<script lang="ts">
  /**
   * How a subject is assessed (credit, exam, points), on a page of its own:
   * the subject's `info` chapter alone, without the chapter list of the study
   * text (the user found the list confusing here, 2026-11-21).
   */
  import { api, errorMessage } from "../lib/api";
  import Icon from "../lib/components/Icon.svelte";
  import Prose from "../lib/components/Prose.svelte";
  import { fill, t } from "../lib/i18n/index.svelte";
  import { live } from "../lib/live.svelte";
  import { render, type Rendered } from "../lib/markdown";
  import { nav } from "../lib/nav.svelte";
  import { subjectColor } from "../lib/palette";
  import { theme } from "../lib/theme.svelte";

  const subject = $derived(live.subject(nav.subject) ?? live.defaultSubject);
  const color = $derived(subjectColor(subject?.color, theme.effective === "dark"));

  let rendered = $state<Rendered | null>(null);
  let error = $state<string | null>(null);
  let article = $state<HTMLElement>();

  $effect(() => {
    const s = subject?.id;
    const c = subject?.info;
    rendered = null;
    error = null;
    if (!s || !c) return;
    let cancelled = false;
    api.chapter(s, c).then(
      (payload) => {
        if (cancelled) return;
        // The page has its own heading.
        rendered = render(payload.markdown.replace(/^# .+\n+/, ""), { subject: s, svgs: payload.svgs });
        article?.closest("main")?.scrollTo({ top: 0 });
      },
      (e) => {
        if (!cancelled) error = errorMessage(e);
      },
    );
    return () => {
      cancelled = true;
    };
  });
</script>

{#if subject}
  <div class="view narrow" style:--c={color}>
    <button class="back" onclick={() => subject && nav.open(subject.id)}>
      <Icon name="chevron-left" size={14} />
      {subject.code} · {subject.name}
    </button>
    <header class="head">
      <div class="eyebrow"><span class="dot"></span><span class="code">{subject.code}</span></div>
      <h1>{t().subject.info}</h1>
    </header>

    {#if !subject.info}
      <div class="card empty">{t().subject.none}</div>
    {:else if error}
      <div class="card empty">{fill(t().study.loadFailed, { error })}</div>
    {:else if rendered}
      <div class="card body">
        <Prose html={rendered.html} tag="article" bind:article />
      </div>
    {:else}
      <div class="card empty">{t().common.loading}</div>
    {/if}
  </div>
{/if}

<style>
  .narrow {
    max-width: 860px !important;
  }
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
  .body {
    padding: 18px 28px 24px;
  }
</style>
