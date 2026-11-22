# Content format

Everything the app teaches lives in `content/`: one folder per subject, plain
text files only (TOML, Markdown, SVG, and PNG/JPG/WebP where a drawing will
not do). The app embeds this folder at build time; debug builds read it from
the disk instead, so edits show after **Reload content** (Settings → Content)
without a rebuild.

`cargo run -p studyvut-cli -- check` validates the whole folder and CI runs
the same check. Fix every error it reports before committing content.
Every commit that adds or corrects material adds its line to `CHANGELOG.md`
under `## Unreleased` ([versioning.md](versioning.md)).

Originals (PDF, slides, photos, protocols) never go in here; a PDF, Office
file or archive in `content/` is an error. The user's own submitted work is
the exception: it goes, as submitted, in `submissions/`
([submissions.md](submissions.md)). They go in
`inbox/<subject>/`, which git ignores, since this repository is public. See
[inbox/README.md](../inbox/README.md).

```
content/
  programs.toml                the study programmes (obory)
  bpc-fy1/                     subject id: the course code in lower case
    subject.toml               the subject, its custom quiz sets
    chapters/
      01-newtonovy-zakony.md   study text, in file name order
    quizzes/
      mechanika.toml           a question bank; also a quiz set of its own
    images/
      tri-kvadry.svg           figures for chapters and questions
```

All texts are in Czech; the format's keys are English, like the code.

## programs.toml

The programmes the Dashboard offers in its **Obor** menu, in this order:

```toml
[[program]]
id = "met"                      # [a-z0-9-]+; subjects refer to it
code = "BPC-MET"
name = "Mikroelektronika a technologie"
```

## subject.toml

```toml
code = "BPC-FY1"                # as in the study system
name = "Fyzika 1"
semester = "ZS 2026/2027"       # subjects are grouped by this, newest first
year = 1                        # the study year (ročník), 1 to 6; optional
programs = ["met"]              # programmes that have it (programs.toml ids);
                                # leave out for a subject of every programme
color = "blue"                  # blue, green, pink, gold, violet, teal, orange, graphite
description = "Mechanika a elektrostatika, přednášky a laboratoře."   # optional
info = "prakt-01-jak-projit-predmetem"   # optional: the chapter on how the subject is
                                # assessed (credit, exam, points), shown on its own
                                # page "Hodnocení předmětu" and left out of the
                                # chapter list of the study text

# Custom quiz sets, optional, shown first and in this order.
[[set]]
id = "old"                      # [a-z0-9-]+, unique within the subject
name = "Staré testy"
description = "otázky z fotek a e-learningu"
tags = ["old"]                  # every question with any of these tags, in file order

[[set]]
id = "exam"
name = "Test nanečisto"
description = "15 náhodných otázek z obou témat"
pick = [
  { quiz = "mechanika", count = 7 },        # random questions from a quiz file
  { quiz = "elektrostatika", count = 8 },
]
shuffle = true                  # default: true for `pick`, false for `tags`
```

A set has either `tags` or `pick`. Every set's best score is remembered.

Sets the app adds by itself, after the custom ones:

- one per quiz file: all its questions, shuffled;
- **Moje chyby**: questions whose latest answer was wrong;
- **K opakování**: questions due for review today (spaced repetition: a
  question answered right comes back after 1, 2, 4, 8, then 16 days; a wrong
  answer sends it back to the start);
- per chapter, **Procvičit kapitolu**: the questions of the chapter's `topics`.

## Quiz files: quizzes/*.toml

The file name (without `.toml`) is the quiz id, `[a-z0-9-]+`.

```toml
name = "Mechanika"
description = "vše z mechaniky, zamíchané"     # optional

# A choice question.
[[question]]
id = "M1"                         # unique within the subject; NEVER change or reuse it,
                                  # the progress of every user is stored under it
topic = "Newtonovy zákony"        # groups questions; chapters practise by topic
source = "Starý test 2023"        # optional, shown as a chip
tags = ["old"]                    # optional
text = """
Tři stejné kvádry o hmotnosti $M$ táhneme silou $T$. Jaká síla napíná lanko mezi A a B?
"""
figure = "images/tri-kvadry.svg"  # optional, shown under the text
options = ["$T$", "$2T/3$", "$T/3$", "$T/2$", "$0$"]
answer = 2                        # index of the right option, from 0
layout = "grid"                   # optional: options in two columns (short options, figures)
explanation = """
Všechna tělesa mají stejné zrychlení $a = T/(3M)$. Lanko A–B táhne jen kvádr A:
$$F_{AB} = M a = \\frac{T}{3}$$
"""
notes = [                         # optional, one per option: why it is wrong
  "To je síla, kterou táhneme C.",
  "Tolik táhne lanko B–C.",
  "",                             # "" for the right one or when there is nothing to say
  "Kvádry jsou tři, ne dva.",
  "Kvádry zrychlují, síla nulová není.",
]
warning = "Pozor na jednotky."    # optional, shown highlighted under the explanation

# A numeric question.
[[question]]
id = "E12"
kind = "numeric"
topic = "Gaussova věta"
text = "Jaký je tok intenzity uzavřenou plochou kolem náboje $-2\\,\\mathrm{nC}$?"
value = -226                      # the right answer
tolerance = 2                     # accepted: value ± tolerance
unit = "V·m"
sign_hint = "Tok, který do uzavřené plochy vstupuje, je záporný."   # optional:
                                  # shown when the answer is right except for the sign
explanation = "$$\\Phi = Q/\\varepsilon_0 = \\dots$$"
```

Rules the check enforces:

- ids are `[A-Za-z0-9_-]+`, unique within the subject;
- a choice question has 2 to 8 options, `answer` points at one of them, and
  `notes` (if given) has one entry per option;
- a numeric question has a finite `value`, `tolerance` ≥ 0 and no `options`;
- every referenced image exists;
- `$` signs pair up in every text.

## Chapters: chapters/*.md

Markdown, sorted by file name, so start names with `01-`, `02-`, … The file
name without `.md` is the chapter id. A TOML front matter between `+++` lines
comes first:

```markdown
+++
title = "Newtonovy zákony"        # optional, else the first "# " heading
topics = ["Newtonovy zákony"]     # optional: quiz topics for "Procvičit kapitolu"
+++

Úvodní odstavec…

## Soustava těles spojených lanky

…
```

`##` and `###` headings make the chapter's contents in the sidebar; their
ids come from the text, or set one with `## Nadpis {#id}`.

## Markdown, in chapters and in every quiz text

- CommonMark with GFM tables, `**tučně**`, `*kurzíva*`, lists, `kód`.
- **Math**: `$…$` inline, `$$…$$` on its own line (KaTeX). In TOML strings
  write a backslash twice (`\\frac`), or use literal strings `'''…'''`, where
  one is enough. Write a dollar sign that is no math as `\$`.
  A `$$…$$` formula never runs out of its box: separate several results on
  one line with `\qquad` (or `,\quad`), and the app puts them under each
  other when the column is narrow; a long equation wraps after `=`, `+`
  or `−`. No `\tag` (the app sets formulas in display style, not display
  mode, which KaTeX needs for it).
- **Images**: `![popis](images/x.svg "Titulek pod obrázkem")`. The path is
  relative to the subject folder. SVGs are inlined, so they follow the
  light and dark theme (see below); PNG, JPG and WebP are shown as they are.
- **Callouts**: a block quote whose first line is `[!kind] Title`:

  ```markdown
  > [!key] Nejdůležitější věta testu
  > „konstantní rychlostí“ ⇔ výsledná síla = 0
  ```

  `key` (the thing to remember), `trap` (a common mistake), `example`
  (a worked example), `note` (anything else). The title is optional.
- Code blocks with a language (` ```c `) for programming subjects.
- Inline HTML only where Markdown has nothing (`<sub>`, `<sup>`, `<kbd>`).

## SVG figures

Figures are inlined into the page and drawn with the app's figure styles, so
they never contain colours of their own. The root element:

```svg
<svg viewBox="0 0 430 110" width="430" class="fig" role="img" aria-label="Tři kvádry spojené lanky">
```

Classes (the vocabulary of the original BFY1 trainer):

| Class | What |
|---|---|
| `s`, `s1` | lines, 2 and 1.3 px |
| `d` | dashed helper line |
| `h` | hatching (ground, ceiling) |
| `f` | force arrow (accent colour), use with `marker-end="url(#m-f)"` |
| `fl` | field line, `marker-end="url(#m-fl)"` |
| `v` | velocity and other vectors, `marker-end="url(#m-v)"` |
| `badl` | a wrong or opposing direction, `marker-end="url(#m-badl)"` |
| `blk`, `gf` | filled bodies (blocks, wedges, spheres) |
| `lq` | liquid; `pl` an accent area |
| `pt`, `ptA`, `ptB` | points: ink, accent, wrong |
| `plate`, `plateA` | capacitor plates |
| text: `it`, `sm`, `mut`, `acc`, `flt`, `bt` | italic (quantities), small, muted, accent, field colour, wrong |

Text is set in the math font by default. The arrow markers `m-f`, `m-fl`,
`m-v` and `m-badl` are defined once by the app; figures only refer to them.
