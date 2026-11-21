<script lang="ts">
  /** The subject menu of the Study and Quiz pages (visible subjects only). */
  import { t } from "../i18n/index.svelte";
  import { live } from "../live.svelte";

  let { value, onchange }: { value: string | undefined; onchange: (id: string) => void } = $props();
</script>

{#if live.visibleSubjects.length > 1}
  <select
    class="select"
    aria-label={t().study.subject}
    {value}
    onchange={(e) => onchange(e.currentTarget.value)}
  >
    {#each live.visibleSubjects as s (s.id)}
      <option value={s.id}>{s.code} · {s.name}</option>
    {/each}
  </select>
{/if}
