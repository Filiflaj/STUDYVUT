# Submitted work

The user's own submitted work (lab protocols, homework, projects) lives in
`submissions/`, next to `content/`, **as submitted**: the original PDF and,
where there are any, its sources and measured data. The app shows it per
subject on the Study page ("Odevzdané práce") and opens the files in their
system apps.

**Everything in `submissions/` is public.** It is committed to this public
repository and built into every copy of the app, like `content/`. Git keeps
it forever: deleting a file later does not take it back from the history,
from forks or from downloaded releases. So:

- only the user's own work goes here, never a classmate's without their
  written consent (a classmate's protocol in the inbox stays in the inbox);
- there is no "private" flag: work that is not to be published stays out of
  the repository;
- no corrected copies with the teacher's marks and no grades.

`cargo run -p studyvut-cli -- check` validates `submissions/` together with
`content/`, and CI runs the same check: every error fails the build.

```
submissions/
  README.md                    what this folder is (required: the app embeds the folder)
  bpc-fy1/                     subject id, as in content/ (the subject must exist there)
    lab-10/                    the work's id: [a-z0-9-]+, e.g. lab-<task>, du-<n>, projekt-<name>
      submission.toml          what the work is, and every file in the folder
      protokol.pdf             the document as submitted
  bpc-mpe/
    lab-01/
      submission.toml
      protokol.pdf
      data/smycka-permalloy-ff-50hz.xls
```

Debug builds read `submissions/` from the disk (edits show after **Reload
content**); release builds embed it. `STUDYVUT_SUBMISSIONS_DIR` points both
the app and the CLI at another folder.

## submission.toml

```toml
title = "Tíhové zrychlení (reverzní kyvadlo)"   # as on the protocol
kind = "protocol"              # protocol, homework, project, test, other
task = 10                      # optional: the task's number or code as the course
                               # names it, a number or text ("L-1", "DU3")
measured = 2026-10-01          # optional, a TOML date: when it was measured
submitted = 2026-10-08         # optional, a TOML date
summary = '''
Poloha závaží, při níž je doba kmitu kolem obou os stejná; pak
$g = 4\pi^2 \ell_r / T^2$.
'''                            # optional, Markdown: what it was about, what to watch out for
chapters = ["lab-10-tihove-zrychleni", "lab-00-zpracovani-mereni"]
                               # optional: chapter ids of the subject, shown as links
topics = ["Úloha 10: reverzní kyvadlo"]
                               # optional: quiz topics; "Procvičit témata" asks their questions

[[file]]                       # one per file of the folder, in the order shown
path = "protokol.pdf"          # relative to the work's folder; subfolders are fine
role = "final"                 # final (as submitted), source, data, other
label = "Protokol"             # optional, shown instead of the file name
```

The summary is Markdown with math like every text of the content (write a
backslash twice in `"…"` strings, or use `'''…'''` literal strings). Dates
are TOML dates (`2026-10-08`, no quotes); a quoted `"2026-10-08"` works too.

The app lists a subject's work newest first (by `submitted`, undated last).

## Files

- **Every file in the folder is listed** in `submission.toml`. A file that is
  not listed would be published and built into the app without being shown,
  so it is an error, as is a folder without `submission.toml` and a file
  outside a work's folder (only `README.md` may sit at the top).
- **Names**: lower case ASCII letters, digits, `-`, `_` and `.`; no spaces,
  no accents, and no personal names (the folder says what it is):
  `protokol.pdf`, `vypocty.docx`, `data/mereni.xlsx`.
- **Types**: documents (`pdf`, `docx`, `doc`, `odt`), spreadsheets (`xlsx`,
  `xls`, `ods`, `csv`), slides (`pptx`, `ppt`, `odp`), images (`png`, `jpg`,
  `webp`), text (`txt`, `md`, `tex`) open in their system app. Sources (`c`,
  `h`, `cpp`, `py`, `m`, `ipynb`, `json`, `toml`, `bib`, `cls`, `sty`, `asc`,
  KiCad files) may be published but the app only shows them in their folder
  or saves a copy: the system would run a script instead of opening it.
  Anything else (programs, archives) is an error.
- **Size**: over 10 MB a file is a warning (every copy of the app carries
  it; is the scan's resolution too high?), over 50 MB an error (GitHub
  warns from 50 MB and refuses 100 MB). The check prints the total.

## Roles

| Role | What |
|---|---|
| `final` | The document as submitted. Every work should have one (a warning otherwise). |
| `source` | What it was made from: LaTeX, Word, a spreadsheet with the calculations. |
| `data` | Measured or exported data (the lab software's `.xls`, CSV). |
| `other` | Anything else worth keeping with it (an appendix). |

## Rules the check enforces

- the subject folder is a subject of `content/`, both folder names are
  `[a-z0-9-]+`;
- `title` is not empty, `kind` and `role` are known values, unknown keys are
  errors (a typo such as `submited` is not silently dropped);
- `visibility` or `private` is an error: everything here is public;
- dates are real `YYYY-MM-DD` dates; `submitted` before `measured` is a warning;
- every listed file exists, is listed once, has a safe relative path and an
  allowed type; every file of the folder is listed;
- chapters exist (a missing one is left out with a warning); a topic no
  question has is a warning;
- `$` signs pair up in the summary.

And `content/` stays plain text: a PDF, an Office file, an archive or a
`submission.toml` anywhere in `content/` is an error.

## In the app

- **Study page**: a subject with submitted work gets a switch "Kapitoly |
  Odevzdané práce" above its chapter list (a subject with work but no
  chapters shows the work directly). A work shows its kind, task, dates,
  files, summary, links to its chapters and "Procvičit témata", which starts
  a quiz with the questions of its topics.
- **Opening a file**: the app copies the embedded file to
  `<data folder>/cache/submissions/<subject>/<id>/` (rewritten when a newer
  build carries a different file) and hands it to the system: Preview,
  Edge or Acrobat, the Linux PDF viewer, Word, Excel. "Ukázat ve složce"
  reveals the copy; "Uložit kopii" saves the file where the user chooses.
  The window names a file by its index, never by a path.
- The app does not show PDFs inside its window: WebKitGTK (Linux) has no
  PDF viewer, so an inline view would need pdf.js; the system viewer works
  the same on all three platforms.

## Adding a submission (for Claude)

1. The user drops the work into `inbox/<subject>/` (or any inbox folder)
   and asks for it to be added.
2. **Whose is it?** Check the file name, the PDF's author metadata
   (`pdfinfo`) and the name on the first page. Anything by someone else stays
   in the inbox and is reported to the user; never copy it.
3. Pick the id (`lab-<task>` with two digits, `du-<n>`, `projekt-<name>`)
   and create `submissions/<subject>/<id>/`.
4. Copy, do not move, the files **unchanged** (the original as submitted),
   renamed to the naming rules: `protokol.pdf`, `vypocty.docx`,
   `data/<what>.xlsx`. No editing, no recompression, no OCR.
5. Write `submission.toml`: the title, task and dates from the document's
   header (the date in a file name may be the measuring date, not the
   submission date), a short summary in your own words, links to the
   subject's chapters and quiz topics of that task (check the ids).
6. Before committing, tell the user what personal data the files carry
   (their student ID, partners' names and IDs, signatures on scanned pages,
   names in the file metadata) unless they already decided for this kind of
   document.
7. Run `cargo run -p studyvut-cli -- check` until it reports 0 errors; it
   prints the submissions' total size.
8. Add a line to `CHANGELOG.md` ("Odevzdané práce: protokol k úloze 10").
