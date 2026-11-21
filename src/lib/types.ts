/** Types of what the Rust side sends (snake_case, as serialized). */

export type ThemePref = "system" | "light" | "dark";
export type LanguagePref = "system" | "cs" | "en";
export type Lang = "cs" | "en";

/** Subject colours: the TOWER Bridge chart palette (palette.ts). */
export const COLORS = ["blue", "green", "pink", "gold", "violet", "teal", "orange", "graphite"] as const;
export type Color = (typeof COLORS)[number];

export interface Settings {
  version: number;
  theme: ThemePref;
  language: LanguagePref;
  daily_goal: number;
  hidden_subjects: string[];
  last_seen_version: string | null;
}

export type ContentSource = { kind: "embedded" } | { kind: "folder"; path: string };

export interface AppInfo {
  version: string;
  data_dir: string;
  content: ContentSource;
  os: string;
  system_language: Lang;
  debug: boolean;
}

export interface ProblemCount {
  errors: number;
  warnings: number;
}

export interface Problem {
  severity: "error" | "warning";
  path: string;
  message: string;
}

export interface Progress {
  questions: number;
  seen: number;
  correct: number;
  wrong: number;
  due: number;
}

export type SetSpec =
  | { kind: "custom"; id: string }
  | { kind: "quiz"; id: string }
  | { kind: "mistakes" }
  | { kind: "due" }
  | { kind: "chapter"; id: string }
  | { kind: "questions"; ids: string[] };

export interface SetView {
  spec: SetSpec;
  kind: "custom" | "quiz" | "mistakes" | "due";
  key: string;
  /** Empty for `mistakes` and `due`, which the app names. */
  name: string;
  description: string | null;
  count: number;
  best: number | null;
}

export interface ChapterView {
  id: string;
  title: string;
  topics: string[];
  questions: number;
}

export interface SubjectView {
  id: string;
  code: string;
  name: string;
  semester: string;
  color: Color;
  description: string | null;
  chapters: ChapterView[];
  sets: SetView[];
  progress: Progress;
}

export interface SessionRow {
  id: number;
  subject: string;
  set_key: string;
  started: number;
  finished: number | null;
  total: number;
  correct: number;
}

export interface Overview {
  subjects: SubjectView[];
  problems: ProblemCount;
  /** The local day number (days since 1970-01-01 in the user's zone). */
  today: number;
  answered_today: number;
  correct_today: number;
  streak: number;
  recent: SessionRow[];
}

export interface Snapshot {
  app: AppInfo;
  settings: Settings;
  overview: Overview;
  first_run: boolean;
}

export interface ChapterPayload {
  subject: string;
  id: string;
  title: string;
  topics: string[];
  markdown: string;
  svgs: Record<string, string>;
  questions: number;
}

export type QuestionKind =
  | { kind: "choice"; options: string[]; answer: number; notes: string[] | null; grid: boolean }
  | { kind: "numeric"; value: number; tolerance: number; unit: string; sign_hint: string | null };

export type Question = {
  id: string;
  topic: string;
  source: string | null;
  tags: string[];
  text: string;
  figure: string | null;
  explanation: string;
  warning: string | null;
} & QuestionKind;

export interface SessionPayload {
  id: number;
  subject: string;
  key: string;
  scored: boolean;
  questions: Question[];
  svgs: Record<string, string>;
}

export type Response = { kind: "choice"; index: number } | { kind: "numeric"; value: number };

export interface Verdict {
  correct: boolean;
  sign_only: boolean;
}

export interface SessionResult {
  session: SessionRow;
  best: number | null;
  new_best: boolean;
}

export interface DayActivity {
  day: number;
  answered: number;
  correct: number;
}

export interface QuestionRow {
  subject: string;
  id: string;
  quiz: string;
  topic: string;
  text: string;
  answered: number;
  correct: number;
  last_correct: boolean | null;
  last_ts: number | null;
  box: number | null;
  due_day: number | null;
}
