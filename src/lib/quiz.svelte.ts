import { api, errorCode, errorMessage } from "./api";
import { live } from "./live.svelte";
import type { Question, Response, SessionPayload, SessionRow, SetSpec, Verdict } from "./types";

/**
 * The running quiz, kept while the user visits other pages. It behaves as
 * the original BFY1 trainer: one question at a time, instant feedback with
 * the explanation of every option, a result with the missed questions.
 */
class QuizSession {
  payload = $state<SessionPayload | null>(null);
  spec = $state<SetSpec | null>(null);
  /** The set's display name, when the page cannot derive it (chapter, selection). */
  name = $state<string | undefined>(undefined);
  index = $state(0);
  score = $state(0);
  missed = $state<string[]>([]);
  /** The answer to the current question, once given. */
  answer = $state<{ response: Response; verdict: Verdict } | null>(null);
  /** The finished quiz: right answers of all. */
  result = $state<SessionRow | null>(null);
  busy = $state(false);
  /** `empty_set` or a message, when starting or answering failed. */
  error = $state<string | null>(null);

  get active(): boolean {
    return this.payload !== null;
  }

  get done(): boolean {
    return this.result !== null;
  }

  get question(): Question | undefined {
    return this.payload?.questions[this.index];
  }

  get total(): number {
    return this.payload?.questions.length ?? 0;
  }

  get last(): boolean {
    return this.index >= this.total - 1;
  }

  async start(subject: string, spec: SetSpec, name?: string): Promise<void> {
    if (this.busy) return;
    this.busy = true;
    this.error = null;
    try {
      const payload = await api.startSession(subject, spec);
      this.payload = payload;
      this.spec = spec;
      this.name = name;
      this.index = 0;
      this.score = 0;
      this.missed = [];
      this.answer = null;
      this.result = null;
    } catch (e) {
      this.error = errorCode(e) === "empty_set" ? "empty_set" : errorMessage(e);
    } finally {
      this.busy = false;
    }
  }

  async respond(response: Response): Promise<void> {
    const p = this.payload;
    const q = this.question;
    if (!p || !q || this.answer || this.busy) return;
    this.busy = true;
    try {
      const verdict = await api.answer(p.id, p.subject, q.id, response);
      this.answer = { response, verdict };
      if (verdict.correct) this.score++;
      else this.missed = [...this.missed, q.id];
    } catch (e) {
      this.error = errorMessage(e);
    } finally {
      this.busy = false;
    }
  }

  async next(): Promise<void> {
    if (!this.answer || this.busy) return;
    if (!this.last) {
      this.index++;
      this.answer = null;
      return;
    }
    const p = this.payload;
    if (!p) return;
    this.busy = true;
    try {
      this.result = await api.finishSession(p.id);
    } catch (e) {
      this.error = errorMessage(e);
    } finally {
      this.busy = false;
    }
    void live.refresh();
  }

  /** Repeat the missed questions of the finished session. */
  retryMissed(): Promise<void> {
    const p = this.payload;
    if (!p || this.missed.length === 0) return Promise.resolve();
    return this.start(p.subject, { kind: "questions", ids: [...this.missed] });
  }

  /** The same set again (newly shuffled). */
  again(): Promise<void> {
    const p = this.payload;
    if (!p || !this.spec) return Promise.resolve();
    return this.start(p.subject, this.spec, this.name);
  }

  /** Leave the quiz (answers so far stay recorded). */
  close() {
    const answered = this.answer !== null || this.index > 0;
    this.payload = null;
    this.spec = null;
    this.answer = null;
    this.result = null;
    this.error = null;
    if (answered) void live.refresh();
  }
}

export const quiz = new QuizSession();
