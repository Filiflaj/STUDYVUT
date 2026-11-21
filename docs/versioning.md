# Versions and releases

STUDYVUT has one version, `X.Y.Z` after [Semantic Versioning](https://semver.org/),
for the app and the study material together. The version changes only in a
release commit, `Release vX.Y.Z`, which `npm run release` makes and tags
`vX.Y.Z`. What changed is listed in [CHANGELOG.md](../CHANGELOG.md), which the
app shows as **Co je nového**.

## One version for app and material

The material in `content/` is built into the app, so a new chapter reaches
classmates only in a new build, and every build they download is a release.
A second, content-only version would move with every app release anyway, and
would split the tag, the release on GitHub, the update check and **Co je
nového** in two. Minor and patch already tell new material from fixes.

Should content ever load from outside the binary (downloaded packs, the
user's own files), the content format will need a version of its own; the
app's version stays as it is.

## Where the version is

| File | Field |
|---|---|
| `package.json`, `package-lock.json` | `version`; in the lock also `packages[""].version` |
| `Cargo.toml` | `[workspace.package] version`, which the core, service and CLI inherit (`version.workspace = true`) |
| `src-tauri/Cargo.toml` | `[package] version`, a literal as in TOWER Bridge; the app reads it at run time (Tauri's package info) |
| `Cargo.lock` | the workspace's own packages |
| `CHANGELOG.md` | the newest `## X.Y.Z (YYYY-MM-DD)` heading |

`src-tauri/tauri.conf.json` has no `version`, so Tauri takes the one in
`src-tauri/Cargo.toml`. Nobody edits these fields by hand: `npm run release`
sets them all, and `npm run check:version` (run by CI) fails when they
disagree.

## Which number to raise

The biggest change since the last release decides.

| Change | Changelog group | Raises |
|---|---|---|
| New subject, chapter, quiz, quiz set or questions | Materiály | minor |
| A chapter extended or rewritten, a clearer explanation, a better figure | Materiály | patch |
| A wrong answer, typo or broken figure corrected; a broken question removed | Opravy | patch |
| A new feature or a visible change of the app | Aplikace | minor |
| A bug or translation fixed, a dependency updated | Opravy | patch |
| A database migration that only adds (tables, columns with defaults) | with its feature | minor |
| Users lose something: progress or settings reset or not carried over, a subject or feature removed | Aplikace, starting with **Pozor:** | major |
| Refactoring, tests, CI, docs, tooling | none | nothing |

- **A patch never migrates the database**, so a patch release is always safe
  to install and to go back from.
- **The content format is not part of the version.** Content lives in this
  repository and is embedded, so a format change converts all of `content/`
  in the same change, together with [content.md](content.md), and the
  version follows what users see. A new optional key that enables a feature
  raises minor with the feature.
- **Question and subject ids never change** ([content.md](content.md)): the
  progress is stored under them, so no change of material costs progress.
- Internal changes alone are no reason for a release; they ship with the
  next one.

### Before 1.0

STUDYVUT starts at 0.1.0. Until 1.0.0, a change that would raise major
raises minor, and its changelog line starts with **Pozor:**. Otherwise 0.x
reads as above: minor brings something new, patch only fixes.
`npm run release -- major` from 0.x makes 1.0.0; release it when the app,
the database and the content format are expected to stay (the user's call).

## The changelog

In Czech: the app shows it to classmates, every one of them reads Czech
(all material is Czech, also under the English UI), and its lines name Czech
chapters and quizzes. One language, so no translation drifts. The headings
that tools read (`Unreleased`, versions, dates) are fixed.

```markdown
# Changelog

## Unreleased

### Materiály

- BPC-FY1: nová kapitola **Práce a energie** a 18 otázek v kvízu
  Mechanika.

## 0.2.0 (2026-12-03)

### Aplikace

- Kvízy: sada **K opakování** s otázkami, které je dnes čas zopakovat.

### Opravy

- BPC-FY1: otázka M7 měla jako správnou odpověď `T/2`, správně je `T/3`.
```

The grammar, which the app relies on and `check:version` enforces:

- The first line is `# Changelog`.
- `## Unreleased` comes first, once: the lines for the next release. The
  release turns it into `## X.Y.Z (YYYY-MM-DD)` (the release's local date)
  and opens a new, empty `## Unreleased` above it.
- Releases follow newest first, `## X.Y.Z (YYYY-MM-DD)`, versions falling,
  dates not rising. No `(unreleased)` headings as in TOWER Bridge.
- Within a section the groups `### Materiály`, `### Aplikace`, `### Opravy`,
  in this order, each only when it has lines.
- Each change is a `- ` item in a group; a long one wraps onto lines indented
  by two spaces. No nested lists, no blank lines inside an item, no text
  outside items.
- Items use inline Markdown only: `**tučně**`, `*kurzíva*`, `` `kód` ``,
  `[odkaz](https://…)`. No HTML, images or math.
- An item starting with `TODO` is a placeholder: the release refuses it.

Writing the lines:

- **In the same commit as the change**, under `## Unreleased`, never later.
  This includes material: converting a lecture into a chapter adds its line.
- For classmates: what they get or what was wrong, not how it was done. One
  line per change; add to an existing line about the same thing.
- Material lines start with the subject code (`BPC-FY1: …`); app lines with
  the area when it helps (`Kvízy: …`).
- Whatever users must know before updating starts with **Pozor:**.

## Releasing

1. Preview: `npm run release -- minor --dry-run` (or `patch`, `major`, or a
   version `X.Y.Z`). It prints the new version, the files, the commit and the
   tag, and whether the real run would refuse.
2. Release: `npm run release -- minor`. The script:
   - refuses a dirty working tree, a detached HEAD, an existing tag, an empty
     `## Unreleased`, a `TODO` item, and versions that disagree already;
   - sets every version field and dates the changelog section;
   - runs `scripts/check-version.mjs vX.Y.Z`;
   - commits `Release vX.Y.Z` (each `--trailer "Key: value"` is appended as a
     trailer) and makes the annotated tag `vX.Y.Z`.

   It never pushes.
3. Check `git show`, then push the branch and the tag together:
   `git push --atomic origin main vX.Y.Z`. Pushing is always a separate,
   manual step.
4. The tag starts the Release workflow (`.github/workflows/release.yml`): it
   checks that the tag matches every version and points at its
   `Release vX.Y.Z` commit, tests, builds macOS, Windows and Linux, and
   creates a **draft** release whose text is the changelog section
   (`node scripts/release.mjs --notes X.Y.Z`). Check the draft and publish
   it.

Before pushing, a release is undone with
`git tag -d vX.Y.Z && git reset --keep HEAD~1`. A pushed tag is never moved
or deleted: fix forward with the next patch.

The first release names its version, since the files already carry 0.1.0:
`npm run release -- 0.1.0`.

Through npm, the arguments go after `--`. Without it npm takes `--dry-run`
and `--trailer` for itself; the script then still runs dry, and refuses a
lost `--trailer`.

## Why not as in TOWER Bridge

TOWER Bridge raises the version with the first change after a release, under
a `## X.Y.Z (unreleased)` heading, and tags by hand. STUDYVUT collects lines
under `## Unreleased` and raises the version only when releasing:

- material changes come several times a week, and which number to raise is
  known only at the release (a typo fixed today, a new chapter tomorrow);
- each version change is then exactly one commit with its tag;
- a build always carries the number of a release that exists (a build of
  `main` between releases carries the last one).

Everything else follows TOWER Bridge: the same version fields, a literal
version in `src-tauri/Cargo.toml`, none in `tauri.conf.json`, the
`check-version.mjs` step in CI and in the release workflow, tags `v*` that
start the release, and a draft release that a person publishes.

## Scripts

- `scripts/release.mjs`: the release, and `--notes X.Y.Z` for a release's
  changelog section. Its pure parts are exported.
- `scripts/check-version.mjs [vX.Y.Z]`: the consistency check; in GitHub
  Actions it takes the tag from the ref. CI runs it on every push, the
  release workflow on the tag.
- `scripts/release.test.mjs`: `node --test scripts/release.test.mjs`.
