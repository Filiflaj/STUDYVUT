//! The user's progress in SQLite: sessions, every answer, and the review
//! state of each question.
//!
//! Questions are keyed by subject and question id, never by position, so the
//! progress survives content updates. A question removed from the content
//! keeps its rows; the app simply no longer asks it.

mod schema;

use std::path::{Path, PathBuf};

pub use rusqlite::Connection;
use rusqlite::{OptionalExtension, params};
use serde::Serialize;

use crate::local_day;
use crate::review::{self, Schedule};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("session {0} does not exist")]
    NoSession(i64),
}

pub type Result<T> = std::result::Result<T, StoreError>;

impl StoreError {
    /// The file is damaged or not a database at all.
    pub fn is_corrupt(&self) -> bool {
        matches!(
            self,
            StoreError::Sqlite(rusqlite::Error::SqliteFailure(e, _))
                if matches!(e.code, rusqlite::ErrorCode::NotADatabase | rusqlite::ErrorCode::DatabaseCorrupt)
        )
    }
}

/// One answer to record.
#[derive(Debug)]
pub struct Answer<'a> {
    pub session: Option<i64>,
    pub subject: &'a str,
    pub question: &'a str,
    pub correct: bool,
    /// The response as JSON.
    pub response: &'a str,
    pub ts: i64,
    /// The user's time zone, minutes east of UTC.
    pub offset_min: i32,
}

/// Where a question stands for the user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct QuestionState {
    pub subject: String,
    pub question: String,
    pub r#box: u8,
    pub due_day: i64,
    pub last_correct: bool,
    pub last_ts: i64,
    pub answered: u32,
}

/// A finished or abandoned session.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionRow {
    pub id: i64,
    pub subject: String,
    pub set_key: String,
    pub started: i64,
    pub finished: Option<i64>,
    pub total: u32,
    pub correct: u32,
}

/// Read-write handle; the app keeps one behind a mutex.
pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        Self::from_connection(Connection::open(path)?)
    }

    /// Open the database; a damaged one is moved aside and a new one
    /// created, rather than running without progress. Returns where the
    /// damaged file went.
    pub fn open_or_replace(path: &Path) -> Result<(Self, Option<PathBuf>)> {
        match Self::open(path) {
            Err(e) if e.is_corrupt() => {
                let stamp = crate::now_ms() / 1000;
                let aside = path.with_extension(format!("corrupt-{stamp}.db"));
                std::fs::rename(path, &aside)?;
                for ext in ["db-wal", "db-shm"] {
                    let _ = std::fs::remove_file(path.with_extension(ext));
                }
                Ok((Self::open(path)?, Some(aside)))
            }
            other => other.map(|s| (s, None)),
        }
    }

    pub fn open_in_memory() -> Result<Self> {
        Self::from_connection(Connection::open_in_memory()?)
    }

    fn from_connection(conn: Connection) -> Result<Self> {
        schema::migrate(&conn)?;
        Ok(Self { conn })
    }

    pub fn start_session(
        &self,
        subject: &str,
        set_key: &str,
        scored: bool,
        total: usize,
        ts: i64,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO sessions (subject, set_key, scored, started, total) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![subject, set_key, scored, ts, total as i64],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Record an answer: the answer row, the session's count and the
    /// question's review schedule, in one transaction.
    pub fn record(&self, a: &Answer<'_>) -> Result<Schedule> {
        let tx = self.conn.unchecked_transaction()?;
        if let Some(id) = a.session {
            let n = tx.execute(
                "UPDATE sessions SET correct = correct + ?2 WHERE id = ?1",
                params![id, a.correct],
            )?;
            if n == 0 {
                return Err(StoreError::NoSession(id));
            }
        }
        tx.execute(
            "INSERT INTO answers (ts, session_id, subject, question, correct, response)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                a.ts, a.session, a.subject, a.question, a.correct, a.response
            ],
        )?;
        let previous: Option<u8> = tx
            .query_row(
                "SELECT box FROM review WHERE subject = ?1 AND question = ?2",
                params![a.subject, a.question],
                |r| r.get(0),
            )
            .optional()?;
        let schedule = review::next(previous, a.correct, local_day(a.ts, a.offset_min));
        tx.execute(
            "INSERT INTO review (subject, question, box, due_day, last_correct, last_ts, answered)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1)
             ON CONFLICT (subject, question) DO UPDATE SET
                box = excluded.box, due_day = excluded.due_day,
                last_correct = excluded.last_correct, last_ts = excluded.last_ts,
                answered = answered + 1",
            params![
                a.subject,
                a.question,
                schedule.r#box,
                schedule.due_day,
                a.correct,
                a.ts
            ],
        )?;
        tx.commit()?;
        Ok(schedule)
    }

    pub fn finish_session(&self, id: i64, ts: i64) -> Result<SessionRow> {
        self.conn.execute(
            "UPDATE sessions SET finished = ?2 WHERE id = ?1 AND finished IS NULL",
            params![id, ts],
        )?;
        self.session(id)?.ok_or(StoreError::NoSession(id))
    }

    pub fn session(&self, id: i64) -> Result<Option<SessionRow>> {
        Ok(self
            .conn
            .query_row(
                "SELECT id, subject, set_key, started, finished, total, correct FROM sessions WHERE id = ?1",
                [id],
                session_row,
            )
            .optional()?)
    }

    /// The review state of every answered question, of one subject or all.
    pub fn question_states(&self, subject: Option<&str>) -> Result<Vec<QuestionState>> {
        let mut stmt = self.conn.prepare(
            "SELECT subject, question, box, due_day, last_correct, last_ts, answered FROM review
             WHERE ?1 IS NULL OR subject = ?1 ORDER BY subject, question",
        )?;
        let rows = stmt.query_map([subject], |r| {
            Ok(QuestionState {
                subject: r.get(0)?,
                question: r.get(1)?,
                r#box: r.get(2)?,
                due_day: r.get(3)?,
                last_correct: r.get(4)?,
                last_ts: r.get(5)?,
                answered: r.get(6)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Forget the progress of one subject, or of everything.
    pub fn reset(&self, subject: Option<&str>) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        for table in ["answers", "review", "sessions"] {
            tx.execute(
                &format!("DELETE FROM {table} WHERE ?1 IS NULL OR subject = ?1"),
                [subject],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Every answer as JSON lines' worth of data, for an export.
    pub fn answers(&self) -> Result<Vec<serde_json::Value>> {
        let mut stmt = self.conn.prepare(
            "SELECT a.ts, a.subject, a.question, a.correct, a.response, s.set_key
             FROM answers a LEFT JOIN sessions s ON s.id = a.session_id ORDER BY a.ts, a.id",
        )?;
        let rows = stmt.query_map([], |r| {
            let response: String = r.get(4)?;
            Ok(serde_json::json!({
                "ts": r.get::<_, i64>(0)?,
                "subject": r.get::<_, String>(1)?,
                "question": r.get::<_, String>(2)?,
                "correct": r.get::<_, bool>(3)?,
                "response": serde_json::from_str::<serde_json::Value>(&response).unwrap_or(serde_json::Value::String(response)),
                "set": r.get::<_, Option<String>>(5)?,
            }))
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }
}

fn session_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<SessionRow> {
    Ok(SessionRow {
        id: r.get(0)?,
        subject: r.get(1)?,
        set_key: r.get(2)?,
        started: r.get(3)?,
        finished: r.get(4)?,
        total: r.get(5)?,
        correct: r.get(6)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DAY_MS;

    fn answer<'a>(session: Option<i64>, question: &'a str, correct: bool, ts: i64) -> Answer<'a> {
        Answer {
            session,
            subject: "fy1",
            question,
            correct,
            response: r#"{"kind":"choice","index":0}"#,
            ts,
            offset_min: 60,
        }
    }

    #[test]
    fn a_session_counts_its_answers() -> Result<()> {
        let store = Store::open_in_memory()?;
        let t0 = 20_000 * DAY_MS;
        let s = store.start_session("fy1", "quiz:mech", true, 3, t0)?;
        store.record(&answer(Some(s), "M1", true, t0 + 1))?;
        store.record(&answer(Some(s), "M2", false, t0 + 2))?;
        store.record(&answer(Some(s), "M3", true, t0 + 3))?;
        let row = store.finish_session(s, t0 + 4)?;
        assert_eq!((row.total, row.correct, row.finished), (3, 2, Some(t0 + 4)));
        let again = store.finish_session(s, t0 + 9)?;
        assert_eq!(
            again.finished,
            Some(t0 + 4),
            "finishing twice keeps the first time"
        );
        assert!(matches!(
            store.record(&answer(Some(999), "M1", true, t0)),
            Err(StoreError::NoSession(999))
        ));
        Ok(())
    }

    #[test]
    fn review_state_follows_the_latest_answer() -> Result<()> {
        let store = Store::open_in_memory()?;
        let t0 = 20_000 * DAY_MS;
        store.record(&answer(None, "M1", true, t0))?;
        store.record(&answer(None, "M1", true, t0 + DAY_MS))?;
        store.record(&answer(None, "M2", false, t0))?;
        let states = store.question_states(Some("fy1"))?;
        let m1 = &states[0];
        assert_eq!((m1.r#box, m1.last_correct, m1.answered), (2, true, 2));
        assert_eq!(m1.due_day, local_day(t0 + DAY_MS, 60) + 2);
        let m2 = &states[1];
        assert_eq!((m2.r#box, m2.last_correct), (1, false));
        assert!(store.question_states(Some("other"))?.is_empty());
        assert_eq!(store.question_states(None)?.len(), 2);
        Ok(())
    }

    #[test]
    fn reset_forgets_one_subject_or_all() -> Result<()> {
        let store = Store::open_in_memory()?;
        store.record(&answer(None, "M1", true, 1))?;
        store.record(&Answer {
            subject: "el1",
            ..answer(None, "E1", true, 2)
        })?;
        store.reset(Some("fy1"))?;
        assert_eq!(store.question_states(None)?.len(), 1);
        assert_eq!(store.answers()?.len(), 1);
        store.reset(None)?;
        assert!(store.question_states(None)?.is_empty());
        Ok(())
    }

    #[test]
    fn a_damaged_file_is_moved_aside() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("studyvut.db");
        std::fs::write(
            &path,
            b"this is not a database, just some text that is long enough",
        )?;
        let (store, aside) = Store::open_or_replace(&path)?;
        assert!(aside.is_some_and(|p| p.exists()));
        store.record(&answer(None, "M1", true, 1))?;
        Ok(())
    }
}
