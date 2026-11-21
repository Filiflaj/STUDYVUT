import { api, errorMessage } from "./api";
import { i18n, resolveLang } from "./i18n/index.svelte";
import { theme } from "./theme.svelte";
import type { AppInfo, Overview, Settings, SubjectView } from "./types";

/** App state mirrored from the backend. */
class Live {
  ready = $state(false);
  app = $state<AppInfo | null>(null);
  settings = $state<Settings | null>(null);
  overview = $state<Overview | null>(null);
  firstRun = $state(false);
  /** Why the last settings save failed, until the next one works. */
  saveError = $state<string | null>(null);
  /** Saves sent and not answered yet. */
  private pending = 0;
  private saving: Promise<void> = Promise.resolve();

  async init(): Promise<void> {
    const s = await api.snapshot();
    this.app = s.app;
    this.setSettings(s.settings);
    this.overview = s.overview;
    this.firstRun = s.first_run;
    this.ready = true;
  }

  /** Fetch the Dashboard data again (after answers, a reset, a reload). */
  async refresh(): Promise<void> {
    try {
      this.overview = await api.overview();
    } catch (e) {
      console.error(e);
    }
  }

  /** Every subject, in the backend's order (newest semester first). */
  get subjects(): SubjectView[] {
    return this.overview?.subjects ?? [];
  }

  /** The subjects of the chosen programme and year, not hidden in the settings. */
  get visibleSubjects(): SubjectView[] {
    const s = this.settings;
    return this.subjects.filter((x) => matches(x, s?.program ?? null, s?.year ?? null, s?.hidden_subjects ?? []));
  }

  subject(id: string | null | undefined): SubjectView | undefined {
    return this.subjects.find((s) => s.id === id);
  }

  /** The subject the Study and Quiz pages open without a choice: the first visible one with material. */
  get defaultSubject(): SubjectView | undefined {
    const visible = this.visibleSubjects;
    return visible.find((s) => s.chapters.length > 0 || s.questions > 0) ?? visible[0];
  }

  /** Store settings and apply theme and language right away. */
  private setSettings(s: Settings) {
    this.settings = s;
    theme.apply(s.theme);
    i18n.set(resolveLang(s.language, this.app?.system_language));
  }

  /**
   * Persist settings: shown at once, saved one after another; only the
   * reply to the last save replaces the page's settings. A failed save puts
   * back what the backend has and says why.
   */
  async save(mutate: (s: Settings) => void): Promise<void> {
    if (!this.settings) return;
    const next = structuredClone($state.snapshot(this.settings)) as Settings;
    mutate(next);
    this.setSettings(next);
    this.pending++;
    const run = this.saving.then(async () => {
      try {
        const saved = await api.saveSettings(structuredClone($state.snapshot(this.settings)) as Settings);
        this.saveError = null;
        if (this.pending === 1) this.setSettings(saved);
      } catch (e) {
        this.saveError = errorMessage(e);
        try {
          this.setSettings((await api.snapshot()).settings);
        } catch {
          // The backend is gone; the page shows the error.
        }
      } finally {
        this.pending--;
      }
    });
    this.saving = run;
    return run;
  }
}

/**
 * Whether a subject belongs to the chosen programme and year. A subject
 * without programmes is in every programme; one without a year in every year.
 */
export function matches(s: SubjectView, program: string | null, year: number | null, hidden: string[]): boolean {
  if (hidden.includes(s.id)) return false;
  if (program && s.programs.length > 0 && !s.programs.includes(program)) return false;
  if (year && s.year !== null && s.year !== year) return false;
  return true;
}

export const live = new Live();
