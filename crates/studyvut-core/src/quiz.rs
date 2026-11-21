//! Evaluating answers and choosing the questions of a set.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::content::{Question, QuestionKind, SetRule, Subject};

/// What the user answered.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Response {
    Choice { index: usize },
    Numeric { value: f64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Verdict {
    pub correct: bool,
    /// A numeric answer right except for its sign.
    pub sign_only: bool,
}

/// Whether `response` answers `question` right. A response of the wrong
/// kind is wrong.
pub fn evaluate(question: &Question, response: &Response) -> Verdict {
    match (&question.kind, response) {
        (QuestionKind::Choice { answer, .. }, Response::Choice { index }) => Verdict {
            correct: index == answer,
            sign_only: false,
        },
        (
            QuestionKind::Numeric {
                value, tolerance, ..
            },
            Response::Numeric { value: x },
        ) => {
            let within = |target: f64| {
                // A little slack for decimal input such as 0.1 + 0.2.
                let slack = 1e-9 * target.abs().max(1.0);
                x.is_finite() && (x - target).abs() <= tolerance + slack
            };
            let correct = within(*value);
            Verdict {
                correct,
                sign_only: !correct && *value != 0.0 && within(-value),
            }
        }
        _ => Verdict {
            correct: false,
            sign_only: false,
        },
    }
}

/// Which questions a session asks.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SetSpec {
    /// A set of `subject.toml`.
    Custom { id: String },
    /// Every question of a quiz file, shuffled.
    Quiz { id: String },
    /// Questions whose latest answer was wrong.
    Mistakes,
    /// Questions due for review.
    Due,
    /// The questions of a chapter's topics.
    Chapter { id: String },
    /// Given questions, in this order (repeating the missed ones of a session).
    Questions { ids: Vec<String> },
}

impl SetSpec {
    /// How sessions name the set in the database (`custom:old`, `quiz:mechanika`, …).
    pub fn key(&self) -> String {
        match self {
            SetSpec::Custom { id } => format!("custom:{id}"),
            SetSpec::Quiz { id } => format!("quiz:{id}"),
            SetSpec::Mistakes => "mistakes".into(),
            SetSpec::Due => "due".into(),
            SetSpec::Chapter { id } => format!("chapter:{id}"),
            SetSpec::Questions { .. } => "retry".into(),
        }
    }

    /// Whether finishing it may set a best score: the sets of the subject
    /// itself, not the personal ones (mistakes, review, a retry).
    pub fn scored(&self) -> bool {
        matches!(self, SetSpec::Custom { .. } | SetSpec::Quiz { .. })
    }
}

/// What the store knows about the user, for the personal sets.
#[derive(Debug, Default)]
pub struct Personal {
    /// Questions whose latest answer was wrong.
    pub mistakes: HashSet<String>,
    /// Questions due for review now.
    pub due: HashSet<String>,
}

/// The question ids of a set, in the order asked. Unknown ids (a question
/// removed from the content) are skipped.
pub fn select(
    subject: &Subject,
    spec: &SetSpec,
    personal: &Personal,
    rng: &mut fastrand::Rng,
) -> Vec<String> {
    let ids = |it: &mut dyn Iterator<Item = &Question>| -> Vec<String> {
        it.map(|q| q.id.clone()).collect()
    };
    match spec {
        SetSpec::Custom { id } => {
            let Some(set) = subject.set(id) else {
                return Vec::new();
            };
            let mut out = match &set.rule {
                SetRule::Tags(tags) => ids(&mut subject
                    .questions()
                    .filter(|q| q.tags.iter().any(|t| tags.contains(t)))),
                SetRule::Pick(picks) => {
                    let mut out = Vec::new();
                    for p in picks {
                        if let Some(quiz) = subject.quiz(&p.quiz) {
                            let mut all = ids(&mut quiz.questions.iter());
                            rng.shuffle(&mut all);
                            all.truncate(p.count);
                            out.extend(all);
                        }
                    }
                    out
                }
            };
            if set.shuffle {
                rng.shuffle(&mut out);
            }
            out
        }
        SetSpec::Quiz { id } => {
            let mut out = subject
                .quiz(id)
                .map(|q| ids(&mut q.questions.iter()))
                .unwrap_or_default();
            rng.shuffle(&mut out);
            out
        }
        SetSpec::Mistakes => {
            let mut out = ids(&mut subject
                .questions()
                .filter(|q| personal.mistakes.contains(&q.id)));
            rng.shuffle(&mut out);
            out
        }
        SetSpec::Due => {
            let mut out = ids(&mut subject.questions().filter(|q| personal.due.contains(&q.id)));
            rng.shuffle(&mut out);
            out
        }
        SetSpec::Chapter { id } => {
            let Some(chapter) = subject.chapter(id) else {
                return Vec::new();
            };
            let mut out = ids(&mut subject
                .questions()
                .filter(|q| chapter.topics.contains(&q.topic)));
            rng.shuffle(&mut out);
            out
        }
        SetSpec::Questions { ids: wanted } => wanted
            .iter()
            .filter(|id| subject.question(id).is_some())
            .cloned()
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{MemFiles, load};

    fn subject() -> Subject {
        let quiz = |prefix: &str| {
            (1..=5)
                .map(|i| {
                    format!(
                        "[[question]]\nid = \"{prefix}{i}\"\ntopic = \"{prefix}\"\ntext = \"t\"\n\
                         options = [\"a\", \"b\"]\nanswer = 0\n{}\n",
                        if i % 2 == 1 { "tags = [\"old\"]" } else { "" }
                    )
                })
                .collect::<String>()
        };
        let files = MemFiles::new()
            .with(
                "s/subject.toml",
                "code = \"X\"\nname = \"X\"\nsemester = \"ZS 2026/2027\"\n\
                 [[set]]\nid = \"old\"\nname = \"Old\"\ntags = [\"old\"]\n\
                 [[set]]\nid = \"exam\"\nname = \"Exam\"\npick = [{ quiz = \"a\", count = 2 }, { quiz = \"b\", count = 3 }]\n",
            )
            .with("s/quizzes/a.toml", format!("name = \"A\"\n{}", quiz("A")))
            .with("s/quizzes/b.toml", format!("name = \"B\"\n{}", quiz("B")))
            .with("s/chapters/01-x.md", "+++\ntitle = \"X\"\ntopics = [\"B\"]\n+++\n");
        let (catalog, problems) = load(&files);
        assert!(problems.is_empty(), "{problems:?}");
        catalog.subjects.into_iter().next().expect("subject")
    }

    #[test]
    fn choice_and_numeric_answers() {
        let s = subject();
        let q = s.question("A1").expect("A1");
        assert!(evaluate(q, &Response::Choice { index: 0 }).correct);
        assert!(!evaluate(q, &Response::Choice { index: 1 }).correct);
        assert!(!evaluate(q, &Response::Numeric { value: 0.0 }).correct);

        let mut n = q.clone();
        n.kind = QuestionKind::Numeric {
            value: -226.0,
            tolerance: 2.0,
            unit: "V·m".into(),
            sign_hint: None,
        };
        assert!(evaluate(&n, &Response::Numeric { value: -224.0 }).correct);
        assert!(!evaluate(&n, &Response::Numeric { value: -223.9 }).correct);
        assert_eq!(
            evaluate(&n, &Response::Numeric { value: 226.0 }),
            Verdict {
                correct: false,
                sign_only: true
            }
        );
        assert!(!evaluate(&n, &Response::Numeric { value: f64::NAN }).correct);

        n.kind = QuestionKind::Numeric {
            value: 0.3,
            tolerance: 0.0,
            unit: String::new(),
            sign_hint: None,
        };
        assert!(
            evaluate(&n, &Response::Numeric { value: 0.1 + 0.2 }).correct,
            "float noise is not a mistake"
        );
    }

    #[test]
    fn sets_pick_the_right_questions() {
        let s = subject();
        let mut rng = fastrand::Rng::with_seed(7);
        let p = Personal::default();

        let old = select(&s, &SetSpec::Custom { id: "old".into() }, &p, &mut rng);
        assert_eq!(
            old,
            vec!["A1", "A3", "A5", "B1", "B3", "B5"],
            "tags keep file order"
        );

        let exam = select(&s, &SetSpec::Custom { id: "exam".into() }, &p, &mut rng);
        assert_eq!(exam.len(), 5);
        assert_eq!(exam.iter().filter(|id| id.starts_with('A')).count(), 2);

        let quiz = select(&s, &SetSpec::Quiz { id: "b".into() }, &p, &mut rng);
        let mut sorted = quiz.clone();
        sorted.sort();
        assert_eq!(sorted, vec!["B1", "B2", "B3", "B4", "B5"]);

        let chapter = select(&s, &SetSpec::Chapter { id: "01-x".into() }, &p, &mut rng);
        assert_eq!(chapter.len(), 5);

        let personal = Personal {
            mistakes: ["A2".to_owned(), "gone".to_owned()].into(),
            due: ["B4".to_owned()].into(),
        };
        assert_eq!(
            select(&s, &SetSpec::Mistakes, &personal, &mut rng),
            vec!["A2"]
        );
        assert_eq!(select(&s, &SetSpec::Due, &personal, &mut rng), vec!["B4"]);

        let retry = SetSpec::Questions {
            ids: vec!["B2".into(), "nope".into(), "A1".into()],
        };
        assert_eq!(select(&s, &retry, &p, &mut rng), vec!["B2", "A1"]);
        assert!(select(&s, &SetSpec::Custom { id: "nope".into() }, &p, &mut rng).is_empty());
    }

    #[test]
    fn set_keys_and_scoring() {
        assert_eq!(SetSpec::Custom { id: "old".into() }.key(), "custom:old");
        assert_eq!(SetSpec::Quiz { id: "mech".into() }.key(), "quiz:mech");
        assert!(SetSpec::Quiz { id: "mech".into() }.scored());
        assert!(!SetSpec::Mistakes.scored());
        assert!(!SetSpec::Questions { ids: vec![] }.scored());
    }
}
