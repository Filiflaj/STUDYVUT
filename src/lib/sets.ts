import { fill, t } from "./i18n/index.svelte";
import type { ChapterView, SetView, SubjectView } from "./types";

/** The chapters of the study text: all but the assessment chapter, which has its own page. */
export function studyChapters(s: SubjectView | undefined): ChapterView[] {
  return (s?.chapters ?? []).filter((c) => c.id !== s?.info);
}

/** "ZS 2026/2027" → "Zimní semestr 2026/2027" (as the study system writes it). */
export function semesterName(s: string): string {
  const m = /^(ZS|LS)\s+(.+)$/i.exec(s.trim());
  if (!m) return s;
  return fill(m[1].toUpperCase() === "ZS" ? t().overview.winter : t().overview.summer, { years: m[2] });
}

/** The display name of a set the backend left unnamed, or of a session's set key. */
export function setName(subject: SubjectView | undefined, key: string, given?: string): string {
  if (given) return given;
  const set = subject?.sets.find((s) => s.key === key);
  if (set?.name) return set.name;
  if (key === "mistakes") return t().quiz.mistakes;
  if (key === "due") return t().quiz.due;
  if (key === "retry") return t().quiz.retry;
  if (key.startsWith("chapter:")) {
    const id = key.slice("chapter:".length);
    const title = subject?.chapters.find((c) => c.id === id)?.title ?? id;
    return fill(t().quiz.chapter, { title });
  }
  return key;
}

/** The description of a set card. */
export function setDescription(set: SetView): string {
  if (set.kind === "mistakes") return t().quiz.mistakesDesc;
  if (set.kind === "due") return t().quiz.dueDesc;
  return set.description ?? "";
}
