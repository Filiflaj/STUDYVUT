<script lang="ts">
  /** Releases from CHANGELOG.md: version, date, and the items per group (in Czech, like the material). */
  import { Marked } from "marked";
  import type { Group, Release } from "../changelog";
  import { fmtDay } from "../format";
  import { fill, t } from "../i18n/index.svelte";

  let { releases, unreleased = [] }: { releases: Release[]; unreleased?: Group[] } = $props();

  // Items are inline Markdown without HTML and without math.
  const marked = new Marked({ gfm: true });
  const html = (item: string) =>
    marked.parseInline(item.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;"), { async: false });

  const label = (key: Group["key"]) => t().whatsNew[key];
  const date = (iso: string) => fmtDay(Date.UTC(+iso.slice(0, 4), +iso.slice(5, 7) - 1, +iso.slice(8, 10)) / 86_400_000, {
    day: "numeric",
    month: "numeric",
    year: "numeric",
  });
</script>

<div class="releases">
  {#if unreleased.length}
    <section>
      <h3>{t().whatsNew.unreleased}</h3>
      {#each unreleased as g (g.key)}
        <div class="group">{label(g.key)}</div>
        <ul>{#each g.items as item, i (i)}<li>{@html html(item)}</li>{/each}</ul>
      {/each}
    </section>
  {/if}
  {#each releases as r (r.version)}
    <section>
      <h3>{fill(t().whatsNew.version, { version: r.version })} <span class="date">{date(r.date)}</span></h3>
      {#each r.groups as g (g.key)}
        <div class="group">{label(g.key)}</div>
        <ul>{#each g.items as item, i (i)}<li>{@html html(item)}</li>{/each}</ul>
      {/each}
    </section>
  {:else}
    {#if !unreleased.length}<p class="muted">{t().whatsNew.none}</p>{/if}
  {/each}
</div>

<style>
  .releases {
    display: flex;
    flex-direction: column;
    gap: 14px;
    -webkit-user-select: text;
    user-select: text;
  }
  h3 {
    font-size: 14px;
  }
  .date {
    margin-left: 6px;
    font-size: 12.5px;
    font-weight: 400;
    color: var(--text-muted);
  }
  .group {
    margin-top: 6px;
    font-size: 11.5px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
  }
  ul {
    margin: 4px 0 0;
    padding-left: 20px;
    color: var(--text-2);
    font-size: 13.5px;
    line-height: 1.5;
  }
  li {
    margin: 2px 0;
  }
  p {
    margin: 0;
  }
</style>
