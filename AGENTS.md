# AGENTS.md

Working notes for everyone who changes this repository, people and coding agents alike. This file covers:

- what the app is and how it is built,
- the commands,
- how study material gets into the app,
- the conventions, versions and releases,
- what earlier sessions learned.

Keep it current. If a change makes a statement here wrong, fix the statement in the same change. If you learn something non-obvious, add it to [Key learnings](#key-learnings).

Open work is tracked in [TODO.md](TODO.md); user-visible changes go in [CHANGELOG.md](CHANGELOG.md) (see [Versions and releases](#versions-and-releases)).

## What this is

STUDYVUT is a desktop study app (Tauri 2, Rust, Svelte 5) for the subjects of the user's study programme (MET, FEKT VUT). It:

- shows the study texts of every subject (Markdown with KaTeX formulas, SVG figures that follow the theme, callouts);
- runs quizzes that behave like the original BFY1 trainer (`../FYZ/trenazer-bfy1.html`): one question at a time, instant feedback, why each wrong option is wrong, numeric answers with a tolerance, a result with the missed questions; unlike the trainer, it shuffles the options of every choice question each time (`src/lib/order.ts`; `keep_order = true` in the content opts out);
- remembers every answer in SQLite and plans repetition (Leitner boxes: 1, 2, 4, 8, 16 days);
- shows the subjects of the chosen programme (obor) and study year (ročník) on a Dashboard, with the questions due for review.

There are **no success statistics** (no charts, scores, best results or streaks): the user removed them as useless on 2026-11-21. Progress is kept only where it drives learning: the latest answer per question (My mistakes) and its review schedule (Due for review). The result screen at the end of a quiz stays, as in the trainer.

Keep its layout, look (sidebar, cards, settings rows) and conventions consistent: new code follows the patterns already here.

The repository is **public** and the app is **desktop only** (both decided by the user, 2026-11-21).

## Commands

Prerequisites: Rust (stable, 1.90+), Node 22.12+.

```sh
npm ci                                  # once
npm run dev                             # browser preview: http://localhost:1420/preview.html
npm run tauri dev                       # the real app with hot reload; reads content/ from the disk
cargo run -p studyvut-cli -- check      # validate content/ (also: npm run content:check)
cargo run -p studyvut-cli -- list       # what the content holds
cargo run -p studyvut-cli -- stats      # the progress the app recorded
```

Run all of these before handing work over; CI runs the same:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -q -p studyvut-cli -- check
npm run check
npm test
npm run check:version
npm run test:scripts
npm run build
```

### The browser preview

`preview.html` runs the whole UI in a browser with a mocked backend (`src/preview.ts`). The study texts and figures are the real ones of `content/bpc-fy1`; questions and progress are made up. Query parameters select the situation: `?tab=overview|subject|info|study|quiz|settings|manual`, `&subject=bpc-fy1`, `&program=met`, `&year=1` (0 for all), `&theme=light|dark`, `&lang=cs|en`, `&fresh=1` (first start), `&whatsnew=1`, `&problems=1`, and `&start=quiz:mechanika&answer=1` (a running quiz with its first question answered), `&work=lab-10` (Study → Odevzdané práce with a work open). Screenshots with headless Chrome:

```sh
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless=new --hide-scrollbars \
  --window-size=1280,900 --virtual-time-budget=6000 --screenshot=out.png \
  "http://localhost:1420/preview.html?tab=study&subject=bpc-fy1"
```

Look at the result after every visible change, in both themes.

## Architecture

```
crates/studyvut-core/       content, quizzes, repetition, progress; no Tauri
  content/                  the content format (docs/content.md): model, raw TOML types, loader and checker,
                            Markdown helpers (front matter, image refs, $ pairing), Files (folder, memory)
  quiz.rs                   evaluating answers (choice, numeric with tolerance and the sign hint), choosing a set's questions
  review.rs                 Leitner boxes on local days
  store/                    SQLite: sessions, answers, review state; PRAGMA user_version migrations (feature "store")
crates/studyvut-service/    what the app and the CLI share; no Tauri
  paths.rs                  the data folder (STUDYVUT_DATA_DIR or the platform's local data folder)
  settings.rs               settings.json, atomic saves, lenient loading, SETTINGS_VERSION
  service.rs                Service: content (reloadable), settings, store; the actions (overview, chapter, sessions, answers, stats)
  views.rs                  what the front ends receive
crates/studyvut-cli/        studyvut-cli: check, list, stats, paths
src-tauri/                  the desktop app (package "studyvut")
  lib.rs                    startup, plugins, single instance, the fatal-error dialog
  content.rs                where content comes from: STUDYVUT_CONTENT_DIR, debug builds the repo's content/, release builds the copy embedded with include_dir
  protocol.rs               content:// (http://content.localhost on Windows): raster images of the content
  commands.rs               Tauri commands; tz = minutes east of UTC for local days
  window.rs, logging.rs, i18n.rs
src/                        Svelte 5 frontend, no router
  App.svelte                shell: sidebar (logo, pages, theme and language, subjects, links), "Co je nového" once after an update
  views/                    Overview (Dashboard: programme and year, clickable subject tiles), Subject (a subject's page: Study,
                            Quizzes, Assessment, Submitted work, Review), Info (Assessment: the `info` chapter alone; it is
                            left out of the Study chapter list), Study, Quizzes, Settings, Manual
  lib/live.svelte.ts        app state from the backend; settings saves
  lib/nav.svelte.ts         the open page, the chosen subject and chapter, quiz requests between pages
  lib/quiz.svelte.ts        the running quiz (kept while other pages are open)
  lib/markdown.ts           marked + KaTeX + callouts + inlined SVGs + heading ids
  lib/changelog.ts          CHANGELOG.md for "Co je nového"
  lib/i18n/                 UI texts in cs and en (the content is Czech either way)
content/programs.toml       the study programmes; subjects carry `year` and `programs`
content/<subject>/          the study material (docs/content.md)
submissions/<subject>/<id>/ the user's own submitted work, as submitted (docs/submissions.md), embedded like content/
inbox/<subject>/            originals to process; git-ignored except README and .gitkeep
docs/                       content.md, versioning.md, manual/
```

- **Content is data, not code.** Everything a subject teaches is in `content/`; the app embeds it at build time (release) or reads it from the disk (debug, `STUDYVUT_CONTENT_DIR`). `content::load` reports every problem as error or warning and keeps loading; a broken question is left out, the rest stays.
- **Progress is keyed by subject and question id**, never by position, so content updates keep it. Never change or reuse a question id.
- **Local days.** Days (repetition, activity, streak) are the user's local days: the webview passes its time zone offset (`tz`) with every call that needs one.
- **SVGs are inlined** so they follow the theme; their class vocabulary (from the BFY1 trainer) is styled in `src/app.css` and the arrow markers are defined once in `App.svelte`. Raster images go through `content://`.

## Study material: from inbox to content

The user drops originals into `inbox/<subject>/` (PDF, slides, photos, notes, protocols) and asks for them to be processed. Turn them into `content/<subject>/` following [docs/content.md](docs/content.md), then run the checker until it reports 0 errors.

- **Public repository.** Never commit originals into `content/`: no textbook or instruction PDFs, figures cropped from them (redraw as SVG), scans with signatures, or personal data. Explain in your own words.
- **The user's own submitted work is the exception** (decided by the user on 2026-11-21): it goes into `submissions/<subject>/<id>/` as submitted, with a `submission.toml` ([docs/submissions.md](docs/submissions.md), "Adding a submission"). The user's own name and student ID stay. **Every other person's name, student ID and signature is redacted** (truly removed: text deleted and image pixels blanked, then verified with `pdftotext` and by looking at every page), and other people's hidden metadata is stripped. Lab partners and teachers count as other people.
- **Other people's work** (a classmate's protocol in the inbox) is never published, redacted or not, without their consent.
- **Calculations** use the standard method taught in class and the numbers exactly as written in the user's worksheet, with intermediate rounding as written; add a short "jak to říct, když se zeptají". Verify every number independently.
- Question ids are stable and unique within a subject; new questions get new ids.
- Every commit that adds or corrects material adds its line to `CHANGELOG.md`.

## Conventions

- Code, comments, docs and content keys in English; content and the changelog in Czech; UI texts in `src/lib/i18n/` (cs and en, the same keys; TypeScript enforces it).
- Rust: `unwrap`/`expect` denied by clippy, `unsafe` denied; errors carry a machine-readable code to the UI (`ServiceError::code`).
- Settings use snake_case serde; `SETTINGS_VERSION` triggers upgrade steps; saves are atomic.
- **State never changes the font weight**: active, current or selected items show it with colour, background, a border or an indicator.
- Subject colours come from a validated palette (`src/lib/palette.ts`); status colours (good, warning, critical) are reserved for right/wrong and problems.
- Charts follow the dataviz method: the activity chart's two steps of the accent were checked with its validator in both themes (`ActivityChart.svelte`).

## Versions and releases

One version, `X.Y.Z`, for the app and the study material; the policy is in
[docs/versioning.md](docs/versioning.md). For people and agents alike:

- **Never edit a version by hand.** It sits in `package.json`,
  `package-lock.json`, `Cargo.toml` (`[workspace.package]`),
  `src-tauri/Cargo.toml`, `Cargo.lock` and the newest `CHANGELOG.md` heading,
  and changes only in a `Release vX.Y.Z` commit made by `npm run release`,
  tagged `vX.Y.Z`. `npm run check:version` (CI) fails when they disagree.
- **Every user-visible change adds its changelog line in the same commit**:
  under `## Unreleased` in `CHANGELOG.md`, in Czech, in the group
  `### Materiály`, `### Aplikace` or `### Opravy` (this order; add the group
  if it is missing). Converted material counts: a new chapter or quiz gets a
  line such as `- BPC-FY1: nová kapitola **Práce a energie**, 18 nových otázek
  v kvízu Mechanika.` Material lines start with the subject code. Write for
  classmates (what they get, not how it was done); wrap at 80 columns with a
  two-space indent. Refactoring, tests, CI and docs get no line. The grammar
  is strict because the app parses it for **Co je nového**; `check:version`
  tells you what is wrong.
- **The biggest change since the last release picks the number:** minor for
  new material, a new feature or a database migration that only adds; patch
  for fixes and polish of material or app. Major when users
  lose progress, settings, a subject or a feature; before 1.0 that is a minor
  whose line starts with `**Pozor:**`. A patch never migrates the database.
  Never change or reuse a question or subject id (docs/content.md).
- **Release only when the user asks.** Preview with
  `npm run release -- <patch|minor|major> --dry-run`, then run it without
  `--dry-run`, adding `--trailer "Key: value"` for each attribution trailer
  the session requires (e.g. `--trailer "Co-Authored-By: …"`). It refuses a
  dirty tree, an empty `## Unreleased` and `TODO` lines: commit the work
  first, then release. Arguments go after `--`.
- **Never push.** The user pushes the release when they want:
  `git push --atomic origin main vX.Y.Z`. The tag starts the Release
  workflow, which makes a draft release with the changelog section as text.
- A release that is wrong and not pushed yet is undone with
  `git tag -d vX.Y.Z && git reset --keep HEAD~1`. A pushed tag is never moved
  or deleted; fix it in the next patch.

## Key learnings

- 2026-11-25. Content written by agents put the right option first: 71 % of
  all choice questions (100 % in EL1, ELS and PC1M) had it under a), and the
  user noticed in a mock test. The app now shuffles options on screen; the
  backend still sees the content index (`quiz.order` maps positions to
  indexes). So never refer to an option by its letter or position in notes
  or explanations (docs/content.md).
- 2026-11-25. On macOS the webview rubber-bands the whole page when a
  scroll reaches an end, even though the page itself is exactly as tall as
  the window (checked in a WKWebView probe): the user saw the window scroll
  "away". `html, body` have `overflow: hidden; overscroll-behavior: none`
  and the sidebar and `main` contain their own overscroll.
- 2026-11-21. Rather than raising the version with the first change after a
  release (`## X.Y.Z (unreleased)`), STUDYVUT collects changelog lines under
  `## Unreleased` and changes the version only in the tagged
  `Release vX.Y.Z` commit made by `scripts/release.mjs`: material changes
  come several times a week and the level is known only at the release, and
  the user wants every version change to be one commit with its tag.
- 2026-11-21. The user asked that the app and its docs name no other project
  it was derived from. The only exception is the MIT notice in
  `THIRD-PARTY-NOTICES.md`, which the license requires; keep it there only.
- 2026-11-21. The BFY1 trainer has 87 questions (6 numeric), not 110: a naive
  `grep "{id:'"` also counts the study-text chapters.
- 2026-11-21. Tauri's `window.confirm` is not reliable in every webview:
  confirmations are inline (Settings → Reset progress).
- 2026-11-22. KaTeX display mode never wraps, and at the default window
  size (Study column 600 px, 520 px in the smallest window) dozens of the
  worked examples ran out of their box behind a scroll bar the user took
  for a cut-off, wrong formula. `markdown.ts` therefore sets `$$…$$` as
  inline math with `\displaystyle` (KaTeX breaks inline math after
  relations and operators) and cuts it at top-level `\qquad` into parts
  that wrap. To find overflowing formulas, render every chapter and quiz
  text at those widths in headless Chrome and compare `scrollWidth` with
  `clientWidth` of each `.katex-display`.
- 2026-11-21. In Markdown, `$…$` must be tokenized before emphasis, or
  `$a_1 b_2$` turns into italics; `markdown.ts` registers math as marked
  extensions for that reason.
