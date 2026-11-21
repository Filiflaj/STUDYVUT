import type { SetSpec } from "./types";

export type Tab = "overview" | "study" | "quiz" | "settings" | "manual";

/** A quiz another page asked to start. */
export interface QuizRequest {
  subject: string;
  spec: SetSpec;
  /** Shown as the set's name (chapter practice, selected questions). */
  name?: string;
}

const SUBJECT_KEY = "studyvut.subject";

function readSubject(): string | null {
  try {
    return localStorage.getItem(SUBJECT_KEY);
  } catch {
    return null;
  }
}

/** Which page is open, and the subject the Study and Quiz pages share. */
class Nav {
  tab = $state<Tab>("overview");
  /** The subject chosen on the Study and Quiz pages (remembered). */
  subject = $state<string | null>(readSubject());
  /** The chapter open on the Study page. */
  chapter = $state<string | null>(null);
  /** A quiz to start when the Quiz page opens. */
  request = $state<QuizRequest | null>(null);

  go(tab: Tab) {
    this.tab = tab;
  }

  selectSubject(id: string) {
    if (this.subject !== id) this.chapter = null;
    this.subject = id;
    try {
      localStorage.setItem(SUBJECT_KEY, id);
    } catch {
      // Private mode: the first subject next time.
    }
  }

  /** Open a subject's study text, optionally at a chapter. */
  study(subject: string, chapter?: string) {
    this.selectSubject(subject);
    if (chapter) this.chapter = chapter;
    this.tab = "study";
  }

  /** Open the Quiz page on a subject, starting a set if given. */
  quiz(subject: string, spec?: SetSpec, name?: string) {
    this.selectSubject(subject);
    this.request = spec ? { subject, spec, name } : null;
    this.tab = "quiz";
  }
}

export const nav = new Nav();
