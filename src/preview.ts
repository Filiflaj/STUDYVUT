// Development preview: the whole UI in a plain browser with a mocked backend
// (no Tauri). npm run dev, then open http://localhost:1420/preview.html
// Query parameters: ?tab=overview|study|quiz|data|settings|manual
//                   &theme=system|light|dark  &lang=system|cs|en
//                   &fresh=1 (first start: no programme or year chosen, the welcome card)
//                   &program=met|  &year=1|2|3|0
//                   &whatsnew=1 (the app was just updated: "Co je nového")
//                   &problems=1 (the content has problems)
// Study texts and figures are the real ones of content/bpc-fy1; the quiz
// questions and the progress are made up.

import { mockIPC } from "@tauri-apps/api/mocks";
import { nav, type Tab } from "./lib/nav.svelte";
import type {
  ChapterPayload,
  LanguagePref,
  Overview,
  Question,
  Response,
  SessionPayload,
  SetSpec,
  Settings,
  Snapshot,
  SubjectView,
  ThemePref,
  Verdict,
} from "./lib/types";

(window as unknown as Record<string, unknown>).__STUDYVUT_PREVIEW__ = true;

const params = new URLSearchParams(location.search);
const fresh = params.get("fresh") === "1";

const CHAPTERS = import.meta.glob<string>("../content/bpc-fy1/chapters/*.md", {
  query: "?raw",
  import: "default",
  eager: true,
});
const SVGS = import.meta.glob<string>("../content/bpc-fy1/images/*.svg", { query: "?raw", import: "default", eager: true });

function chapterOf(path: string) {
  const id = path.split("/").pop()?.replace(/\.md$/, "") ?? "";
  const source = CHAPTERS[path];
  const fm = /^\+\+\+\n([\s\S]*?)\n\+\+\+\n*/.exec(source);
  const front = fm?.[1] ?? "";
  const title = /title\s*=\s*"([^"]*)"/.exec(front)?.[1] ?? /^# (.+)$/m.exec(source)?.[1] ?? id;
  const topics = [...(/topics\s*=\s*\[([^\]]*)\]/.exec(front)?.[1] ?? "").matchAll(/"([^"]*)"/g)].map((m) => m[1]);
  return { id, title, topics, markdown: fm ? source.slice(fm[0].length) : source };
}

const chapters = Object.keys(CHAPTERS).sort().map(chapterOf);
const svg = (name: string) => SVGS[`../content/bpc-fy1/images/${name}`] ?? "";

const QUESTIONS: Question[] = [
  {
    id: "M1",
    topic: "Newtonovy zákony",
    source: "Starý test",
    tags: ["old"],
    text: "Tři stejné kvádry o hmotnosti $M$ táhneme silou $T$. Jaká síla napíná lanko mezi A a B?",
    figure: "images/tri-kvadry.svg",
    explanation: "Všechna tělesa mají stejné zrychlení $a = T/(3M)$. Lanko A–B táhne jen kvádr A:\n$$F_{AB} = M a = \\frac{T}{3}$$",
    warning: null,
    kind: "choice",
    options: ["$T$", "$2T/3$", "$T/3$", "$T/2$", "$0$"],
    answer: 2,
    notes: ["To je síla, kterou táhneme C.", "Tolik táhne lanko B–C.", "", "Kvádry jsou tři.", "Kvádry zrychlují."],
    grid: false,
  },
  {
    id: "X2",
    topic: "Coulombův zákon",
    source: null,
    tags: [],
    text: "Vzdálenost dvou bodových nábojů ztrojnásobíme. Síla mezi nimi:",
    figure: null,
    explanation: "$$F \\propto \\frac{1}{r^2} \\Rightarrow F' = \\frac{F}{3^2} = \\frac{F}{9}$$",
    warning: null,
    kind: "choice",
    options: ["klesne na 1/3", "klesne na 1/9", "vzroste 3×", "vzroste 9×"],
    answer: 1,
    notes: ["Zapomíná na druhou mocninu.", "", "Síla se vzdáleností klesá.", "Síla se vzdáleností klesá."],
    grid: true,
  },
  {
    id: "E15",
    topic: "Gaussova věta",
    source: "Sbírka",
    tags: [],
    text: "Jaký je tok intenzity uzavřenou plochou kolem náboje $-2\\,\\mathrm{nC}$? Výsledek ve $\\mathrm{V\\cdot m}$.",
    figure: null,
    explanation: "$$\\Phi = \\frac{Q}{\\varepsilon_0} = \\frac{-2\\cdot10^{-9}}{8{,}85\\cdot10^{-12}} \\doteq -226\\,\\mathrm{V\\cdot m}$$",
    warning: "Tok, který do plochy vstupuje, je záporný.",
    kind: "numeric",
    value: -226,
    tolerance: 2,
    unit: "V·m",
    sign_hint: "Tok, který do uzavřené plochy vstupuje, je záporný.",
  },
];

function subject(
  id: string,
  code: string,
  name: string,
  color: SubjectView["color"],
  withContent: boolean,
): SubjectView {
  const questions = withContent ? 87 : 0;
  return {
    id,
    code,
    name,
    semester: "ZS 2026/2027",
    year: 1,
    programs: ["met"],
    color,
    description: null,
    chapters: withContent ? chapters.map((c) => ({ id: c.id, title: c.title, topics: c.topics, questions: 4 })) : [],
    sets: withContent
      ? [
          { spec: { kind: "custom", id: "old" }, kind: "custom", key: "custom:old", name: "Staré testy", description: "otázky z fotek a e-learningu", count: 41 },
          { spec: { kind: "custom", id: "exam" }, kind: "custom", key: "custom:exam", name: "Test nanečisto", description: "15 náhodných otázek z obou témat", count: 15 },
          { spec: { kind: "quiz", id: "mechanika" }, kind: "quiz", key: "quiz:mechanika", name: "Mechanika", description: "vše z mechaniky, zamíchané", count: 42 },
          { spec: { kind: "quiz", id: "elektrostatika" }, kind: "quiz", key: "quiz:elektrostatika", name: "Elektrostatika", description: "vše z elektrostatiky, zamíchané", count: 45 },
          { spec: { kind: "mistakes" }, kind: "mistakes", key: "mistakes", name: "", description: null, count: fresh ? 0 : 7 },
          { spec: { kind: "due" }, kind: "due", key: "due", name: "", description: null, count: fresh ? 0 : 5 },
        ]
      : [],
    questions,
    due: fresh || !withContent ? 0 : 5,
  };
}

const subjects: SubjectView[] = [
  subject("bpc-el1", "BPC-EL1", "Elektrotechnika 1", "orange", false),
  subject("bpc-els", "BPC-ELS", "Elektrotechnický seminář", "orange", false),
  subject("bpc-fy1", "BPC-FY1", "Fyzika 1", "blue", true),
  subject("bpc-fys", "BPC-FYS", "Fyzikální seminář", "blue", false),
  subject("bpc-ma1", "BPC-MA1", "Matematika 1", "violet", false),
  subject("bpc-mas", "BPC-MAS", "Matematický seminář", "violet", false),
  subject("bpc-mpe", "BPC-MPE", "Materiály pro elektrotechniku", "gold", false),
  subject("bpc-pc1m", "BPC-PC1M", "Počítače a programování 1", "green", false),
  subject("bpc-tde", "BPC-TDE", "Technická dokumentace", "teal", false),
];

const overview = (): Overview => ({
  programs: [{ id: "met", code: "BPC-MET", name: "Mikroelektronika a technologie" }],
  subjects,
  problems: params.get("problems") === "1" ? { errors: 1, warnings: 2 } : { errors: 0, warnings: 0 },
});

let settings: Settings = {
  version: 1,
  theme: (params.get("theme") as ThemePref) ?? "system",
  language: (params.get("lang") as LanguagePref) ?? "system",
  program: params.get("program") ?? (fresh ? null : "met"),
  year: params.has("year") ? Number(params.get("year")) || null : fresh ? null : 1,
  hidden_subjects: [],
  // ?whatsnew=1: seen an older version, so "Co je nového" opens (once there are releases).
  last_seen_version: params.get("whatsnew") === "1" ? "0.0.0" : "0.1.0",
};

const snapshot = (): Snapshot => ({
  app: {
    version: "0.1.0",
    data_dir: "~/Library/Application Support/io.github.filiflaj.studyvut",
    content: { kind: "folder", path: "/Users/student/STUDYVUT/content" },
    os: "macos",
    system_language: "cs",
    debug: true,
  },
  settings,
  overview: overview(),
  first_run: fresh,
});

function verdict(q: Question, r: Response): Verdict {
  if (q.kind === "choice" && r.kind === "choice") return { correct: r.index === q.answer, sign_only: false };
  if (q.kind === "numeric" && r.kind === "numeric") {
    const ok = Math.abs(r.value - q.value) <= q.tolerance;
    return { correct: ok, sign_only: !ok && Math.abs(r.value + q.value) <= q.tolerance };
  }
  return { correct: false, sign_only: false };
}

let session = 10;

mockIPC((cmd, args) => {
  const a = (args ?? {}) as Record<string, unknown>;
  switch (cmd) {
    case "get_snapshot":
      return snapshot();
    case "get_overview":
      return overview();
    case "get_chapter": {
      const c = chapters.find((x) => x.id === a.chapter) ?? chapters[0];
      const svgs: Record<string, string> = {};
      for (const m of c.markdown.matchAll(/!\[[^\]]*\]\((images\/[^)\s]+\.svg)/g)) svgs[m[1]] = svg(m[1].slice(7));
      const payload: ChapterPayload = { subject: "bpc-fy1", ...c, svgs, questions: 4 };
      return payload;
    }
    case "start_session": {
      const spec = a.spec as SetSpec;
      if (spec.kind === "due" && fresh) throw { code: "empty_set", message: "the set has no questions" };
      const questions = spec.kind === "questions" ? QUESTIONS.filter((q) => spec.ids.includes(q.id)) : QUESTIONS;
      const payload: SessionPayload = {
        id: ++session,
        subject: String(a.subject),
        key: spec.kind === "custom" || spec.kind === "quiz" ? `${spec.kind}:${spec.id}` : spec.kind === "questions" ? "retry" : spec.kind === "chapter" ? `chapter:${spec.id}` : spec.kind,
        questions,
        svgs: { "images/tri-kvadry.svg": svg("tri-kvadry.svg") },
      };
      return payload;
    }
    case "answer": {
      const q = QUESTIONS.find((x) => x.id === a.question);
      return q ? verdict(q, a.response as Response) : { correct: false, sign_only: false };
    }
    case "finish_session":
      return { id: Number(a.session), subject: "bpc-fy1", set_key: "quiz:mechanika", started: Date.now() - 300_000, finished: Date.now(), total: 3, correct: 2 };
    case "get_problems":
      return [
        { severity: "error", path: "bpc-fy1/quizzes/mechanika.toml", message: "question M3: answer 5 is out of range (options 0 to 4)" },
        { severity: "warning", path: "bpc-fy1/chapters/04-treni.md", message: 'topic "Tření" has no questions' },
      ];
    case "save_settings":
      settings = a.settings as Settings;
      return settings;
    case "reload_content":
      return overview().problems;
    case "reset_progress":
    case "open_data_folder":
    case "open_link":
    case "open_url":
      return null;
    case "export_progress":
      return "/Users/student/Downloads/studyvut-pokrok.json";
    default:
      console.warn("preview: unmocked command", cmd, a);
      return null;
  }
});

const tab = params.get("tab") as Tab | null;
if (tab) nav.go(tab);
const chosen = params.get("subject");
if (chosen) nav.selectSubject(chosen);

await import("./main");

// ?start=quiz:mechanika (or custom:old, mistakes, …) starts a quiz;
// &answer=<option index> answers its first question.
const start = params.get("start");
if (start) {
  const { quiz } = await import("./lib/quiz.svelte");
  const [kind, id] = start.split(":");
  nav.go("quiz");
  await quiz.start("bpc-fy1", (id ? { kind, id } : { kind }) as SetSpec);
  const answer = params.get("answer");
  if (answer !== null) await quiz.respond({ kind: "choice", index: Number(answer) });
}
