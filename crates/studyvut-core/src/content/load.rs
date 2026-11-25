//! Reading `content/` into a catalog, checking as it goes.

use std::collections::{BTreeMap, HashMap, HashSet};

use super::files::Files;
use super::markdown::{dollars_balanced, first_heading, image_refs, split_front_matter};
use super::model::{
    Catalog, Chapter, Document, Program, Question, QuestionKind, Quiz, SetDef, SetRule, Subject,
};
use super::raw::{
    RawDocument, RawFrontMatter, RawKind, RawLayout, RawPrograms, RawQuestion, RawQuiz, RawSubject,
};
use super::{Problem, Severity, is_image};

const MAX_OPTIONS: usize = 8;

/// Read every subject in `files` (`<subject>/subject.toml` and what is next
/// to it). Returns what loaded and everything wrong, in file order.
pub fn load(files: &dyn Files) -> (Catalog, Vec<Problem>) {
    let paths = files.list();
    let mut by_subject: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut problems = Vec::new();
    for path in &paths {
        if is_original(path) && !is_document(path) {
            problems.push(error(path, ORIGINAL));
            continue;
        }
        match path.split_once('/') {
            Some((subject, _)) => by_subject.entry(subject).or_default().push(path),
            None if path.ends_with(".md") || path == PROGRAMS => {}
            None => problems.push(warning(path, "a file outside any subject folder")),
        }
    }
    let programs = load_programs(files, &mut problems);

    let mut subjects = Vec::new();
    for (id, paths) in by_subject {
        let mut ctx = Ctx {
            files,
            subject: id,
            paths: paths.iter().copied().collect(),
            programs: &programs,
            problems: &mut problems,
        };
        if let Some(s) = ctx.subject(&paths) {
            subjects.push(s);
        }
    }
    subjects.sort_by(|a, b| {
        semester_key(&b.semester)
            .cmp(&semester_key(&a.semester))
            .then_with(|| a.code.cmp(&b.code))
    });
    (Catalog { programs, subjects }, problems)
}

/// The study programmes, at the top of the content folder.
const PROGRAMS: &str = "programs.toml";

fn load_programs(files: &dyn Files, problems: &mut Vec<Problem>) -> Vec<Program> {
    let Some(text) = files.read_string(PROGRAMS) else {
        return Vec::new();
    };
    let raw: RawPrograms = match toml::from_str(&text) {
        Ok(r) => r,
        Err(e) => {
            problems.push(error(PROGRAMS, toml_message(&e)));
            return Vec::new();
        }
    };
    let mut out: Vec<Program> = Vec::new();
    for p in raw.programs {
        if !valid_id(&p.id, &[]) || p.id.chars().any(|c| c.is_ascii_uppercase()) {
            problems.push(error(
                PROGRAMS,
                format!("program {:?}: id must be [a-z0-9-]+", p.id),
            ));
        } else if out.iter().any(|o| o.id == p.id) {
            problems.push(error(PROGRAMS, format!("program {:?}: duplicate id", p.id)));
        } else {
            out.push(Program {
                id: p.id,
                code: p.code,
                name: p.name,
            });
        }
    }
    out
}

/// Newer semesters sort higher: "ZS 2026/2027" < "LS 2026/2027" < "ZS 2027/2028".
fn semester_key(s: &str) -> (u32, u8, String) {
    let year = s
        .split(|c: char| !c.is_ascii_digit())
        .find(|p| p.len() == 4)
        .and_then(|y| y.parse().ok())
        .unwrap_or(0);
    let term = u8::from(s.trim_start().to_ascii_uppercase().starts_with("LS"));
    (year, term, s.to_owned())
}

const ORIGINAL: &str = "documents and archives never go in content/ (plain text only); \
     the user's submitted work goes in submissions/ (docs/submissions.md), originals in inbox/";

/// Documents and archives: they belong in inbox/ or submissions/, never in
/// the content (text the app embeds).
const DOCUMENTS: &[&str] = &[
    "pdf", "doc", "docx", "xls", "xlsx", "xlsm", "ppt", "pptx", "odt", "ods", "odp", "zip", "7z",
    "rar",
];

/// A PDF in a subject's `files/`: study material the app shows, which the
/// subject must list as a `[[document]]` (checked with the subject).
fn is_document(path: &str) -> bool {
    let mut parts = path.split('/');
    parts.next().is_some()
        && parts.next() == Some("files")
        && parts
            .next()
            .is_some_and(|name| name.to_ascii_lowercase().ends_with(".pdf"))
        && parts.next().is_none()
}

fn is_original(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    let ext = name.rsplit_once('.').map(|(_, e)| e.to_ascii_lowercase());
    name == "submission.toml" || ext.is_some_and(|e| DOCUMENTS.contains(&e.as_str()))
}

fn error(path: &str, message: impl Into<String>) -> Problem {
    Problem {
        severity: Severity::Error,
        path: path.to_owned(),
        message: message.into(),
    }
}

fn warning(path: &str, message: impl Into<String>) -> Problem {
    Problem {
        severity: Severity::Warning,
        path: path.to_owned(),
        message: message.into(),
    }
}

fn valid_id(id: &str, extra: &[char]) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || extra.contains(&c))
}

/// The first line of a TOML error, which is where and what.
fn toml_message(e: &toml::de::Error) -> String {
    let msg = e.message().trim().to_owned();
    match e.span() {
        Some(span) => format!("{msg} (at byte {})", span.start),
        None => msg,
    }
}

struct Ctx<'a> {
    files: &'a dyn Files,
    subject: &'a str,
    paths: HashSet<&'a str>,
    programs: &'a [Program],
    problems: &'a mut Vec<Problem>,
}

impl Ctx<'_> {
    fn subject(&mut self, paths: &[&str]) -> Option<Subject> {
        let id = self.subject;
        let toml_path = format!("{id}/subject.toml");
        if !valid_id(id, &[]) || id.chars().any(|c| c.is_ascii_uppercase()) {
            self.problems.push(error(
                &toml_path,
                format!("subject folder {id:?} must be lower case letters, digits and -"),
            ));
            return None;
        }
        let Some(text) = self.files.read_string(&toml_path) else {
            self.problems.push(error(
                &toml_path,
                "missing (every subject folder needs one)",
            ));
            return None;
        };
        let raw: RawSubject = match toml::from_str(&text) {
            Ok(r) => r,
            Err(e) => {
                self.problems.push(error(&toml_path, toml_message(&e)));
                return None;
            }
        };

        let mut quizzes = Vec::new();
        let mut chapters = Vec::new();
        let mut seen_ids: HashMap<String, String> = HashMap::new();
        for path in paths {
            let rel = &path[id.len() + 1..];
            if let Some(name) = rel.strip_prefix("quizzes/") {
                if let Some(quiz_id) = name.strip_suffix(".toml").filter(|n| !n.contains('/')) {
                    if let Some(q) = self.quiz(path, quiz_id, &mut seen_ids) {
                        quizzes.push(q);
                    }
                } else {
                    self.problems
                        .push(warning(path, "not a quiz file (quizzes/<id>.toml)"));
                }
            } else if let Some(name) = rel.strip_prefix("chapters/") {
                if let Some(chapter_id) = name.strip_suffix(".md").filter(|n| !n.contains('/')) {
                    if let Some(c) = self.chapter(path, chapter_id) {
                        chapters.push(c);
                    }
                } else if !is_image(name) {
                    self.problems
                        .push(warning(path, "not a chapter (chapters/<id>.md)"));
                }
            } else if rel.starts_with("files/") {
                if !raw.documents.iter().any(|d| d.file == rel) {
                    self.problems.push(error(
                        path,
                        "not listed as a [[document]] in subject.toml (only PDFs made for \
                         the study material go in files/, never originals of the school)",
                    ));
                }
            } else if rel != "subject.toml" && !rel.starts_with("images/") {
                self.problems.push(warning(
                    path,
                    "unknown file; expected subject.toml, chapters/, quizzes/, images/ or files/",
                ));
            }
        }
        let documents = self.documents(&toml_path, raw.documents);

        if let Some(year) = raw.year
            && !(1..=6).contains(&year)
        {
            self.problems
                .push(error(&toml_path, format!("year {year} is not 1 to 6")));
        }
        for p in &raw.programs {
            if !self.programs.iter().any(|x| &x.id == p) {
                self.problems.push(error(
                    &toml_path,
                    format!("program {p:?} is not in {PROGRAMS}"),
                ));
            }
        }
        let mut subject = Subject {
            id: id.to_owned(),
            code: raw.code,
            name: raw.name,
            semester: raw.semester,
            year: raw.year.filter(|y| (1..=6).contains(y)),
            programs: raw.programs,
            color: raw.color,
            description: raw.description,
            info: None,
            chapters,
            quizzes,
            sets: Vec::new(),
            documents,
        };
        subject.sets = self.sets(&toml_path, raw.sets, &subject);
        if let Some(info) = raw.info {
            if subject.chapter(&info).is_some() {
                subject.info = Some(info);
            } else {
                self.problems.push(error(
                    &toml_path,
                    format!("info: no chapter {info:?} (chapters/{info}.md)"),
                ));
            }
        }
        self.check_topics(&subject);
        Some(subject)
    }

    fn documents(&mut self, path: &str, raw: Vec<RawDocument>) -> Vec<Document> {
        let mut out: Vec<Document> = Vec::new();
        for d in raw {
            let at = format!("document {:?}", d.file);
            let full = format!("{}/{}", self.subject, d.file);
            if !is_document(&full) {
                self.problems
                    .push(error(path, format!("{at}: must be files/<name>.pdf")));
            } else if !self.paths.contains(full.as_str()) {
                self.problems
                    .push(error(path, format!("{at}: {} does not exist", d.file)));
            } else if d.title.trim().is_empty() {
                self.problems
                    .push(error(path, format!("{at}: needs a title")));
            } else if out.iter().any(|o| o.file == d.file) {
                self.problems
                    .push(error(path, format!("{at}: listed twice")));
            } else {
                let size = self.files.size(&full).unwrap_or(0);
                out.push(Document {
                    file: d.file,
                    title: d.title,
                    description: d.description,
                    size,
                });
            }
        }
        out
    }

    fn sets(&mut self, path: &str, raw: Vec<super::raw::RawSet>, subject: &Subject) -> Vec<SetDef> {
        let mut out: Vec<SetDef> = Vec::new();
        for s in raw {
            let at = format!("set {:?}", s.id);
            if !valid_id(&s.id, &[]) {
                self.problems
                    .push(error(path, format!("{at}: id must be [a-z0-9-]+")));
                continue;
            }
            if out.iter().any(|o| o.id == s.id) {
                self.problems
                    .push(error(path, format!("{at}: duplicate id")));
                continue;
            }
            let rule = match (s.tags, s.pick) {
                (Some(tags), None) => {
                    let matched = subject
                        .questions()
                        .filter(|q| q.tags.iter().any(|t| tags.contains(t)))
                        .count();
                    if matched == 0 {
                        self.problems.push(warning(
                            path,
                            format!("{at}: no question has any of the tags {tags:?}"),
                        ));
                    }
                    SetRule::Tags(tags)
                }
                (None, Some(pick)) => {
                    let mut ok = true;
                    for p in &pick {
                        match subject.quiz(&p.quiz) {
                            None => {
                                self.problems.push(error(
                                    path,
                                    format!("{at}: no quiz file quizzes/{}.toml", p.quiz),
                                ));
                                ok = false;
                            }
                            Some(q) if q.questions.len() < p.count => {
                                self.problems.push(warning(
                                    path,
                                    format!(
                                        "{at}: picks {} from {}, which has only {}",
                                        p.count,
                                        p.quiz,
                                        q.questions.len()
                                    ),
                                ));
                            }
                            Some(_) => {}
                        }
                    }
                    if !ok {
                        continue;
                    }
                    SetRule::Pick(pick)
                }
                _ => {
                    self.problems.push(error(
                        path,
                        format!("{at}: needs either `tags` or `pick`, not both or neither"),
                    ));
                    continue;
                }
            };
            let shuffle = s.shuffle.unwrap_or(matches!(rule, SetRule::Pick(_)));
            out.push(SetDef {
                id: s.id,
                name: s.name,
                description: s.description,
                rule,
                shuffle,
            });
        }
        out
    }

    fn quiz(
        &mut self,
        path: &str,
        id: &str,
        seen_ids: &mut HashMap<String, String>,
    ) -> Option<Quiz> {
        if !valid_id(id, &[]) || id.chars().any(|c| c.is_ascii_uppercase()) {
            self.problems
                .push(error(path, "quiz file names must be [a-z0-9-]+.toml"));
            return None;
        }
        let text = self.files.read_string(path)?;
        let raw: RawQuiz = match toml::from_str(&text) {
            Ok(r) => r,
            Err(e) => {
                self.problems.push(error(path, toml_message(&e)));
                return None;
            }
        };
        let mut questions = Vec::new();
        for (i, value) in raw.questions.into_iter().enumerate() {
            let label = value
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| format!("question {s}"))
                .unwrap_or_else(|| format!("question #{}", i + 1));
            let rq: RawQuestion = match value.try_into() {
                Ok(q) => q,
                Err(e) => {
                    self.problems
                        .push(error(path, format!("{label}: {}", toml_message(&e))));
                    continue;
                }
            };
            if let Some(prev) = seen_ids.get(&rq.id) {
                self.problems
                    .push(error(path, format!("{label}: id already used in {prev}")));
                continue;
            }
            if let Some(q) = self.question(path, &label, rq) {
                seen_ids.insert(q.id.clone(), path.to_owned());
                questions.push(q);
            }
        }
        if questions.is_empty() {
            self.problems.push(warning(path, "no questions"));
        }
        Some(Quiz {
            id: id.to_owned(),
            name: raw.name,
            description: raw.description,
            questions,
        })
    }

    fn question(&mut self, path: &str, label: &str, q: RawQuestion) -> Option<Question> {
        let fail = |problems: &mut Vec<Problem>, msg: String| {
            problems.push(error(path, format!("{label}: {msg}")));
        };
        if !valid_id(&q.id, &['_']) {
            fail(self.problems, "id must be [A-Za-z0-9_-]+".into());
            return None;
        }
        if q.topic.trim().is_empty() {
            fail(self.problems, "topic is empty".into());
            return None;
        }
        if q.text.trim().is_empty() {
            fail(self.problems, "text is empty".into());
            return None;
        }
        let kind = match q.kind {
            RawKind::Choice => {
                if q.value.is_some()
                    || q.tolerance.is_some()
                    || q.unit.is_some()
                    || q.sign_hint.is_some()
                {
                    fail(
                        self.problems,
                        "value, tolerance, unit and sign_hint are for kind = \"numeric\"".into(),
                    );
                    return None;
                }
                let Some(options) = q.options else {
                    fail(self.problems, "options missing".into());
                    return None;
                };
                if !(2..=MAX_OPTIONS).contains(&options.len()) {
                    fail(
                        self.problems,
                        format!("needs 2 to {MAX_OPTIONS} options, has {}", options.len()),
                    );
                    return None;
                }
                let Some(answer) = q.answer else {
                    fail(self.problems, "answer missing".into());
                    return None;
                };
                if answer >= options.len() {
                    fail(
                        self.problems,
                        format!(
                            "answer {answer} is out of range (options 0 to {})",
                            options.len() - 1
                        ),
                    );
                    return None;
                }
                if let Some(notes) = &q.notes
                    && notes.len() != options.len()
                {
                    fail(
                        self.problems,
                        format!("{} notes for {} options", notes.len(), options.len()),
                    );
                    return None;
                }
                QuestionKind::Choice {
                    options,
                    answer,
                    notes: q.notes,
                    grid: q.layout == Some(RawLayout::Grid),
                    keep_order: q.keep_order.unwrap_or(false),
                }
            }
            RawKind::Numeric => {
                if q.options.is_some()
                    || q.answer.is_some()
                    || q.notes.is_some()
                    || q.layout.is_some()
                    || q.keep_order.is_some()
                {
                    fail(
                        self.problems,
                        "options, answer, notes, layout and keep_order are for choice questions"
                            .into(),
                    );
                    return None;
                }
                let Some(value) = q.value.filter(|v| v.is_finite()) else {
                    fail(self.problems, "value missing or not a number".into());
                    return None;
                };
                let tolerance = q.tolerance.unwrap_or(0.0);
                if !(tolerance.is_finite() && tolerance >= 0.0) {
                    fail(self.problems, "tolerance must be a number ≥ 0".into());
                    return None;
                }
                QuestionKind::Numeric {
                    value,
                    tolerance,
                    unit: q.unit.unwrap_or_default(),
                    sign_hint: q.sign_hint,
                }
            }
        };
        let question = Question {
            id: q.id,
            topic: q.topic.trim().to_owned(),
            source: q.source,
            tags: q.tags,
            text: q.text.trim().to_owned(),
            figure: q.figure,
            explanation: q.explanation.trim().to_owned(),
            warning: q.warning,
            kind,
        };
        self.check_texts(path, label, &question);
        Some(question)
    }

    /// Images exist and math delimiters pair up in every text of a question.
    fn check_texts(&mut self, path: &str, label: &str, q: &Question) {
        let mut texts: Vec<(&str, &str)> = vec![("text", &q.text), ("explanation", &q.explanation)];
        if let Some(w) = &q.warning {
            texts.push(("warning", w));
        }
        if let QuestionKind::Choice { options, notes, .. } = &q.kind {
            texts.extend(options.iter().map(|o| ("option", o.as_str())));
            if let Some(n) = notes {
                texts.extend(n.iter().map(|o| ("note", o.as_str())));
            }
        }
        for (what, text) in texts {
            if !dollars_balanced(text) {
                self.problems
                    .push(error(path, format!("{label}: unpaired $ in the {what}")));
            }
            for img in image_refs(text) {
                self.check_image(path, &format!("{label}: {what}"), &img);
            }
        }
        if let Some(fig) = &q.figure {
            self.check_image(path, &format!("{label}: figure"), fig);
        }
    }

    fn check_image(&mut self, path: &str, at: &str, image: &str) {
        if image.contains("://") {
            self.problems.push(error(
                path,
                format!("{at}: {image} is a web address; images must be in images/"),
            ));
            return;
        }
        let full = format!("{}/{image}", self.subject);
        if !is_image(image) {
            self.problems
                .push(error(path, format!("{at}: {image} is not an image")));
        } else if !self.paths.contains(full.as_str()) {
            self.problems
                .push(error(path, format!("{at}: {image} does not exist")));
        }
    }

    fn chapter(&mut self, path: &str, id: &str) -> Option<Chapter> {
        let source = self.files.read_string(path)?;
        let (front, body) = split_front_matter(&source);
        let fm: RawFrontMatter = match front.map(toml::from_str).transpose() {
            Ok(fm) => fm.unwrap_or_default(),
            Err(e) => {
                self.problems
                    .push(error(path, format!("front matter: {}", toml_message(&e))));
                RawFrontMatter::default()
            }
        };
        let title = fm
            .title
            .or_else(|| first_heading(body).map(str::to_owned))
            .unwrap_or_else(|| {
                self.problems.push(warning(
                    path,
                    "no title (front matter `title` or a `# ` heading)",
                ));
                id.to_owned()
            });
        if !dollars_balanced(body) {
            self.problems.push(error(path, "unpaired $"));
        }
        for img in image_refs(body) {
            self.check_image(path, "image", &img);
        }
        Some(Chapter {
            id: id.to_owned(),
            title,
            topics: fm.topics,
            markdown: body.to_owned(),
        })
    }

    /// Chapter topics that no question has practise nothing.
    fn check_topics(&mut self, subject: &Subject) {
        let topics = subject.topics();
        for c in &subject.chapters {
            for t in &c.topics {
                if !topics.contains(&t.as_str()) {
                    self.problems.push(warning(
                        &format!("{}/chapters/{}.md", subject.id, c.id),
                        format!("topic {t:?} has no questions"),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::MemFiles;

    const SUBJECT: &str = r#"
code = "BPC-FY1"
name = "Fyzika 1"
semester = "ZS 2026/2027"
color = "blue"

[[set]]
id = "old"
name = "Staré testy"
tags = ["old"]

[[set]]
id = "exam"
name = "Test nanečisto"
pick = [{ quiz = "mech", count = 1 }]
"#;

    const QUIZ: &str = r#"
name = "Mechanika"

[[question]]
id = "M1"
topic = "Newtonovy zákony"
tags = ["old"]
text = "Kolik je $1 + 1$?"
figure = "images/a.svg"
options = ["1", "2"]
answer = 1
explanation = "Prostě $2$."
notes = ["Málo.", ""]

[[question]]
id = "N1"
kind = "numeric"
topic = "Newtonovy zákony"
text = "Kolik je $5/2$?"
value = 2.5
tolerance = 0.005
unit = ""
"#;

    fn files() -> MemFiles {
        MemFiles::new()
            .with("fy1/subject.toml", SUBJECT)
            .with("fy1/quizzes/mech.toml", QUIZ)
            .with("fy1/images/a.svg", "<svg/>")
            .with(
                "fy1/chapters/01-newton.md",
                "+++\ntopics = [\"Newtonovy zákony\"]\n+++\n# Newtonovy zákony\n\n## První {#prvni}\n",
            )
            .with("fy1/.DS_Store", "")
    }

    #[test]
    fn a_valid_subject_loads_without_problems() {
        let (catalog, problems) = load(&files());
        assert_eq!(problems, vec![]);
        let s = catalog.subject("fy1").expect("subject");
        assert_eq!(s.code, "BPC-FY1");
        assert_eq!(s.quizzes.len(), 1);
        assert_eq!(s.questions().count(), 2);
        assert_eq!(s.chapters[0].title, "Newtonovy zákony");
        assert_eq!(s.chapters[0].topics, vec!["Newtonovy zákony"]);
        assert_eq!(s.sets.len(), 2);
        assert!(
            s.set("exam").is_some_and(|e| e.shuffle),
            "pick sets shuffle by default"
        );
        assert!(s.set("old").is_some_and(|e| !e.shuffle));
        match &s.question("N1").expect("N1").kind {
            QuestionKind::Numeric { value, .. } => assert!((value - 2.5).abs() < 1e-9),
            k => panic!("numeric expected, got {k:?}"),
        }
    }

    fn messages(files: &MemFiles) -> Vec<String> {
        load(files).1.into_iter().map(|p| p.to_string()).collect()
    }

    #[test]
    fn broken_questions_are_reported_and_left_out() {
        let quiz = r#"
name = "Q"
[[question]]
id = "A"
topic = "t"
text = "x"
options = ["1", "2"]
answer = 2

[[question]]
id = "B"
topic = "t"
text = "x"
options = ["1", "2"]
answer = 0
notes = ["only one"]

[[question]]
id = "C"
topic = "t"
text = "$x"
options = ["1", "2"]
answer = 0
figure = "images/missing.svg"

[[question]]
id = "C"
topic = "t"
text = "duplicate"
options = ["1", "2"]
answer = 0

[[question]]
id = "D"
topic = "t"
text = "typo"
options = ["1", "2"]
answer = 0
expalnation = "x"
"#;
        let files = MemFiles::new()
            .with(
                "s/subject.toml",
                "code = \"X\"\nname = \"X\"\nsemester = \"ZS 2026/2027\"",
            )
            .with("s/quizzes/q.toml", quiz);
        let (catalog, _) = load(&files);
        let ids: Vec<_> = catalog.subjects[0]
            .questions()
            .map(|q| q.id.as_str())
            .collect();
        assert_eq!(
            ids,
            vec!["C"],
            "only C loads (with problems that do not drop it)"
        );
        let m = messages(&files);
        let has = |needle: &str| m.iter().any(|x| x.contains(needle));
        assert!(has("question A: answer 2 is out of range"), "{m:?}");
        assert!(has("question B: 1 notes for 2 options"), "{m:?}");
        assert!(has("question C: unpaired $ in the text"), "{m:?}");
        assert!(
            has("question C: figure: images/missing.svg does not exist"),
            "{m:?}"
        );
        assert!(
            has("question C: id already used in s/quizzes/q.toml"),
            "{m:?}"
        );
        assert!(has("question D: unknown field `expalnation`"), "{m:?}");
    }

    #[test]
    fn keep_order_is_for_choice_questions() {
        let quiz = r#"
name = "Q"
[[question]]
id = "A"
topic = "t"
text = "x"
options = ["graf a)", "graf b)"]
answer = 1
keep_order = true

[[question]]
id = "B"
topic = "t"
text = "x"
options = ["1", "2"]
answer = 0

[[question]]
id = "N"
kind = "numeric"
topic = "t"
text = "x"
value = 1.0
tolerance = 0.0
unit = ""
keep_order = true
"#;
        let files = MemFiles::new()
            .with(
                "s/subject.toml",
                "code = \"X\"\nname = \"X\"\nsemester = \"ZS 2026/2027\"",
            )
            .with("s/quizzes/q.toml", quiz);
        let (catalog, _) = load(&files);
        let keep = |id: &str| match catalog.subjects[0].question(id).map(|q| &q.kind) {
            Some(QuestionKind::Choice { keep_order, .. }) => *keep_order,
            k => panic!("choice expected, got {k:?}"),
        };
        assert!(keep("A"));
        assert!(
            !keep("B"),
            "options are shuffled unless the content says so"
        );
        assert!(catalog.subjects[0].question("N").is_none());
        assert!(
            messages(&files)
                .iter()
                .any(|m| m.contains("question N: options, answer, notes, layout and keep_order")),
            "{:?}",
            messages(&files)
        );
    }

    #[test]
    fn sets_must_refer_to_existing_quizzes() {
        let subject = "code = \"X\"\nname = \"X\"\nsemester = \"ZS 2026/2027\"\n\
                       [[set]]\nid = \"e\"\nname = \"E\"\npick = [{ quiz = \"nope\", count = 1 }]\n\
                       [[set]]\nid = \"both\"\nname = \"B\"\n";
        let files = MemFiles::new().with("s/subject.toml", subject);
        let m = messages(&files);
        assert!(
            m.iter()
                .any(|x| x.contains("set \"e\": no quiz file quizzes/nope.toml")),
            "{m:?}"
        );
        assert!(
            m.iter().any(|x| x.contains("set \"both\": needs either")),
            "{m:?}"
        );
    }

    #[test]
    fn subjects_sort_newest_semester_first() {
        let s = |code: &str, sem: &str| {
            format!("code = \"{code}\"\nname = \"n\"\nsemester = \"{sem}\"")
        };
        let files = MemFiles::new()
            .with("a/subject.toml", s("BPC-A", "ZS 2026/2027"))
            .with("b/subject.toml", s("BPC-B", "LS 2026/2027"))
            .with("c/subject.toml", s("BPC-C", "ZS 2027/2028"))
            .with("d/subject.toml", s("BPC-0", "ZS 2026/2027"));
        let (catalog, problems) = load(&files);
        assert_eq!(problems, vec![]);
        let codes: Vec<_> = catalog.subjects.iter().map(|s| s.code.as_str()).collect();
        assert_eq!(codes, vec!["BPC-C", "BPC-B", "BPC-0", "BPC-A"]);
    }

    #[test]
    fn documents_in_the_content_are_errors() {
        let files = files()
            .with("fy1/images/protokol.pdf", "%PDF")
            .with("fy1/lab-10/submission.toml", "title = \"x\"")
            .with("fy1/data.XLSX", "PK")
            .with("prace.zip", "PK");
        let m = messages(&files);
        assert_eq!(m.len(), 4, "{m:?}");
        assert!(
            m.iter()
                .all(|x| x.starts_with("error:") && x.contains("never go in content/")),
            "{m:?}"
        );
        assert!(
            load(&files).0.subject("fy1").is_some(),
            "the rest still loads"
        );
    }

    #[test]
    fn listed_pdfs_in_files_are_documents() {
        let subject = "code = \"X\"\nname = \"X\"\nsemester = \"ZS 2026/2027\"\n\
            [[document]]\nfile = \"files/vzorce.pdf\"\ntitle = \"Vzorce\"\n\
            [[document]]\nfile = \"files/chybi.pdf\"\ntitle = \"Chybí\"\n\
            [[document]]\nfile = \"images/a.pdf\"\ntitle = \"Jinde\"\n";
        let files = MemFiles::new()
            .with("s/subject.toml", subject)
            .with("s/files/vzorce.pdf", "%PDF-1.7")
            .with("s/files/zadani.pdf", "%PDF")
            .with("s/files/zadani.docx", "PK")
            .with("s/images/a.pdf", "%PDF");
        let (catalog, problems) = load(&files);
        let docs = &catalog.subjects[0].documents;
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].file, "files/vzorce.pdf");
        assert_eq!(docs[0].size, 8);
        let m: Vec<String> = problems.iter().map(ToString::to_string).collect();
        let has =
            |path: &str, needle: &str| m.iter().any(|x| x.contains(path) && x.contains(needle));
        assert!(
            has("s/files/zadani.pdf", "not listed as a [[document]]"),
            "{m:?}"
        );
        assert!(has("s/files/zadani.docx", "never go in content/"), "{m:?}");
        assert!(has("s/images/a.pdf", "never go in content/"), "{m:?}");
        assert!(
            has("s/subject.toml", "files/chybi.pdf does not exist"),
            "{m:?}"
        );
        assert!(
            has("s/subject.toml", "images/a.pdf\": must be files/<name>.pdf"),
            "{m:?}"
        );
        assert_eq!(m.len(), 5, "{m:?}");
    }

    #[test]
    fn a_folder_without_subject_toml_is_an_error() {
        let files = MemFiles::new().with("x/quizzes/q.toml", "name = \"q\"");
        let m = messages(&files);
        assert_eq!(
            m,
            vec!["error: x/subject.toml: missing (every subject folder needs one)"]
        );
    }
    #[test]
    fn programs_and_years() {
        let programs = "[[program]]\nid = \"met\"\ncode = \"BPC-MET\"\nname = \"Mikroelektronika a technologie\"\n";
        let s = |extra: &str| {
            format!("code = \"X\"\nname = \"X\"\nsemester = \"ZS 2026/2027\"\n{extra}")
        };
        let files = MemFiles::new()
            .with("programs.toml", programs)
            .with("a/subject.toml", s("year = 1\nprograms = [\"met\"]"))
            .with("b/subject.toml", s("year = 9\nprograms = [\"nope\"]"))
            .with("c/subject.toml", s(""));
        let (catalog, problems) = load(&files);
        assert_eq!(catalog.programs.len(), 1);
        assert_eq!(catalog.programs[0].code, "BPC-MET");
        let a = catalog.subject("a").expect("a");
        assert_eq!(
            (a.year, a.programs.clone()),
            (Some(1), vec!["met".to_owned()])
        );
        let b = catalog.subject("b").expect("b");
        assert_eq!(b.year, None, "an invalid year is dropped");
        let c = catalog.subject("c").expect("c");
        assert!(
            c.year.is_none() && c.programs.is_empty(),
            "both are optional"
        );
        let m: Vec<String> = problems.iter().map(|p| p.to_string()).collect();
        assert!(
            m.iter().any(|x| x.contains("year 9 is not 1 to 6")),
            "{m:?}"
        );
        assert!(
            m.iter()
                .any(|x| x.contains("program \"nope\" is not in programs.toml")),
            "{m:?}"
        );
        assert_eq!(m.len(), 2, "{m:?}");
    }
    #[test]
    fn info_names_an_existing_chapter() {
        let s = |info: &str| {
            format!("code = \"X\"\nname = \"X\"\nsemester = \"ZS 2026/2027\"\ninfo = \"{info}\"\n")
        };
        let files = MemFiles::new()
            .with("a/subject.toml", s("00-pravidla"))
            .with("a/chapters/00-pravidla.md", "# Jak projít předmětem\n")
            .with("b/subject.toml", s("nope"));
        let (catalog, problems) = load(&files);
        assert_eq!(
            catalog.subject("a").and_then(|x| x.info.clone()).as_deref(),
            Some("00-pravidla")
        );
        assert_eq!(catalog.subject("b").and_then(|x| x.info.clone()), None);
        let m: Vec<String> = problems.iter().map(|p| p.to_string()).collect();
        assert_eq!(
            m,
            vec!["error: b/subject.toml: info: no chapter \"nope\" (chapters/nope.md)"]
        );
    }
}
