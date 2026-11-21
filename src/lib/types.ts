/** Types of what the Rust side sends (snake_case, as serialized). */

export type ThemePref = "system" | "light" | "dark";
export type LanguagePref = "system" | "cs" | "en";
export type Lang = "cs" | "en";

/** Subject colours (palette.ts). */
export const COLORS = ["blue", "green", "pink", "gold", "violet", "teal", "orange", "graphite"] as const;
export type Color = (typeof COLORS)[number];

export interface Settings {
  version: number;
  theme: ThemePref;
  language: LanguagePref;
  /** The programme (programs.toml id) whose subjects are shown; null for all. */
  program: string | null;
  /** The study year (ročník) whose subjects are shown; null for all. */
  year: number | null;
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

/** A study programme (obor). */
export interface Program {
  id: string;
  code: string;
  name: string;
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
  /** The study year (ročník), when the content says. */
  year: number | null;
  /** Programme ids; empty for every programme. */
  programs: string[];
  color: Color;
  description: string | null;
  /** The chapter on how the subject is assessed (credit, exam), if any. */
  info: string | null;
  chapters: ChapterView[];
  sets: SetView[];
  questions: number;
  /** Questions due for review today. */
  due: number;
  /** Submitted work of the subject (submissions/), shown on the Study page. */
  submissions: number;
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
  programs: Program[];
  subjects: SubjectView[];
  problems: ProblemCount;
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
  questions: Question[];
  svgs: Record<string, string>;
}

export type Response = { kind: "choice"; index: number } | { kind: "numeric"; value: number };

export interface Verdict {
  correct: boolean;
  sign_only: boolean;
}

export type SubmissionKind = "protocol" | "homework" | "project" | "test" | "other";
export type FileRole = "final" | "source" | "data" | "other";

/** A submitted work in a subject's list (docs/submissions.md). */
export interface SubmissionRow {
  subject: string;
  id: string;
  title: string;
  kind: SubmissionKind;
  task: string | null;
  /** `YYYY-MM-DD`. */
  submitted: string | null;
  files: number;
  size: number;
}

export interface SubmissionFileView {
  /** What the file commands take. */
  index: number;
  path: string;
  role: FileRole;
  label: string | null;
  size: number;
  /** A document the system opens; otherwise only "show in folder" and "save a copy". */
  openable: boolean;
}

export interface SubmissionDetail {
  subject: string;
  id: string;
  title: string;
  kind: SubmissionKind;
  task: string | null;
  measured: string | null;
  submitted: string | null;
  /** Markdown. */
  summary: string | null;
  files: SubmissionFileView[];
  chapters: { id: string; title: string }[];
  topics: { topic: string; questions: number }[];
  /** Question ids of the topics, for `{ kind: "questions", ids }`. */
  practice: string[];
}

