<script lang="ts">
  /** Rendered Markdown: in-page anchors scroll, web links open in the browser, images zoom. */
  import { api } from "../api";
  import { t } from "../i18n/index.svelte";
  import Icon from "./Icon.svelte";

  let {
    html,
    tag = "div",
    article = $bindable(),
    class: cls = "",
  }: { html: string; tag?: "div" | "article"; article?: HTMLElement; class?: string } = $props();

  let zoomed = $state<{ src: string; alt: string } | null>(null);

  function onclick(e: MouseEvent) {
    const target = e.target as Element;
    if (target instanceof HTMLImageElement && target.closest("figure")) {
      zoomed = { src: target.currentSrc || target.src, alt: target.alt };
      return;
    }
    const link = target.closest("a");
    if (!link) return;
    e.preventDefault();
    const href = link.getAttribute("href") ?? "";
    if (href.startsWith("#")) {
      article?.querySelector(`#${CSS.escape(href.slice(1))}`)?.scrollIntoView({ behavior: "smooth", block: "start" });
    } else if (/^https?:\/\//.test(href)) {
      void api.openUrl(href);
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
<svelte:element this={tag} class="prose selectable {cls}" bind:this={article} {onclick} role="document">
  {@html html}
</svelte:element>

{#if zoomed}
  <button class="lightbox" aria-label={t().common.close} onclick={() => (zoomed = null)}>
    <img src={zoomed.src} alt={zoomed.alt} />
    <span class="close" aria-hidden="true"><Icon name="close" size={18} /></span>
  </button>
{/if}

<svelte:window onkeydown={(e) => e.key === "Escape" && (zoomed = null)} />

<style>
  .lightbox {
    position: fixed;
    inset: 0;
    z-index: 50;
    display: grid;
    place-items: center;
    padding: 32px;
    border: none;
    background: rgba(10, 10, 16, 0.72);
    cursor: zoom-out;
  }
  .lightbox img {
    max-width: 100%;
    max-height: 100%;
    border-radius: var(--radius-sm);
    background: #fff;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
  }
  .lightbox .close {
    position: absolute;
    top: 16px;
    right: 16px;
    display: grid;
    place-items: center;
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.16);
    color: #fff;
  }
</style>
