import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import type {
  ChapterPayload,
  Overview,
  Problem,
  ProblemCount,
  Response,
  SessionPayload,
  SessionRow,
  SetSpec,
  Settings,
  Snapshot,
  Verdict,
} from "./types";

/** The user's time zone in minutes east of UTC, for local days. */
export const tz = (): number => -new Date().getTimezoneOffset();

export const api = {
  snapshot: () => invoke<Snapshot>("get_snapshot", { tz: tz() }),
  overview: () => invoke<Overview>("get_overview", { tz: tz() }),
  chapter: (subject: string, chapter: string) => invoke<ChapterPayload>("get_chapter", { subject, chapter }),
  startSession: (subject: string, spec: SetSpec) =>
    invoke<SessionPayload>("start_session", { subject, spec, tz: tz() }),
  answer: (session: number | null, subject: string, question: string, response: Response) =>
    invoke<Verdict>("answer", { session, subject, question, response, tz: tz() }),
  finishSession: (session: number) => invoke<SessionRow>("finish_session", { session }),
  problems: () => invoke<Problem[]>("get_problems"),
  saveSettings: (settings: Settings) => invoke<Settings>("save_settings", { settings }),
  reloadContent: () => invoke<ProblemCount>("reload_content"),
  resetProgress: (subject: string | null) => invoke<void>("reset_progress", { subject }),
  /** Asks where to save; the path written, or null when cancelled. */
  exportProgress: (lang: string) => invoke<string | null>("export_progress", { lang }),
  openDataFolder: () => invoke<void>("open_data_folder"),
  openLink: (target: "repository" | "releases" | "issues") => invoke<void>("open_link", { target }),
  /** An external page in the system browser (http and https only). */
  openUrl: (url: string) => invoke<void>("open_url", { url }),
};

/** URL of a raster image of the content (`bpc-fy1/images/x.png`), served by the app. */
export function contentUrl(path: string): string {
  // The browser preview has no app: the dev server serves the repository.
  if (typeof window !== "undefined" && "__STUDYVUT_PREVIEW__" in window) return `/content/${path}`;
  return convertFileSrc(path, "content");
}

/** Machine-readable code of a command error (e.g. "empty_set"). */
export function errorCode(e: unknown): string | undefined {
  if (e && typeof e === "object" && "code" in e) return String((e as { code: unknown }).code);
  return undefined;
}

/** Message of an error rejected by a Tauri command. */
export function errorMessage(e: unknown): string {
  if (e && typeof e === "object" && "message" in e) return String((e as { message: unknown }).message);
  return String(e);
}
