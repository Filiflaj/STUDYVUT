//! Schema migrations (`PRAGMA user_version` ladder), as in TOWER Bridge.

use rusqlite::Connection;

const MIGRATIONS: &[&str] = &[
    // 1
    "CREATE TABLE sessions (
        id INTEGER PRIMARY KEY,
        subject TEXT NOT NULL,
        set_key TEXT NOT NULL,
        scored INTEGER NOT NULL,
        started INTEGER NOT NULL,
        finished INTEGER,
        total INTEGER NOT NULL,
        correct INTEGER NOT NULL DEFAULT 0
    );
    CREATE INDEX idx_sessions_subject ON sessions(subject, set_key);
    CREATE TABLE answers (
        id INTEGER PRIMARY KEY,
        ts INTEGER NOT NULL,
        session_id INTEGER REFERENCES sessions(id) ON DELETE SET NULL,
        subject TEXT NOT NULL,
        question TEXT NOT NULL,
        correct INTEGER NOT NULL,
        response TEXT NOT NULL
    );
    CREATE INDEX idx_answers_question ON answers(subject, question, ts);
    CREATE INDEX idx_answers_ts ON answers(ts);
    CREATE TABLE review (
        subject TEXT NOT NULL,
        question TEXT NOT NULL,
        box INTEGER NOT NULL,
        due_day INTEGER NOT NULL,
        last_correct INTEGER NOT NULL,
        last_ts INTEGER NOT NULL,
        answered INTEGER NOT NULL,
        PRIMARY KEY (subject, question)
    ) WITHOUT ROWID;",
];

pub fn migrate(conn: &Connection) -> rusqlite::Result<()> {
    apply(conn, MIGRATIONS, MIGRATIONS.len())
}

fn apply(conn: &Connection, migrations: &[&str], target: usize) -> rusqlite::Result<()> {
    let version: i64 = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
    let version = usize::try_from(version).unwrap_or(0);
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.busy_timeout(std::time::Duration::from_secs(5))?;
    for (i, sql) in migrations.iter().enumerate().take(target).skip(version) {
        // The version changes in the same transaction as the schema, so a
        // crash can never leave a migrated schema with the old version.
        let batch = format!("BEGIN; {sql}; PRAGMA user_version = {}; COMMIT;", i + 1);
        if let Err(e) = conn.execute_batch(&batch) {
            if !conn.is_autocommit() {
                let _ = conn.execute_batch("ROLLBACK");
            }
            return Err(e);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version(conn: &Connection) -> rusqlite::Result<i64> {
        conn.pragma_query_value(None, "user_version", |r| r.get(0))
    }

    #[test]
    fn a_new_database_ends_at_the_latest_version() -> rusqlite::Result<()> {
        let conn = Connection::open_in_memory()?;
        migrate(&conn)?;
        assert_eq!(version(&conn)?, MIGRATIONS.len() as i64);
        migrate(&conn)?;
        assert_eq!(
            version(&conn)?,
            MIGRATIONS.len() as i64,
            "a no-op the second time"
        );
        Ok(())
    }

    #[test]
    fn a_failed_migration_changes_nothing() -> rusqlite::Result<()> {
        let conn = Connection::open_in_memory()?;
        let migrations = [
            "CREATE TABLE a (x INTEGER);",
            "CREATE TABLE b (x INTEGER); INSERT INTO nowhere VALUES (1);",
        ];
        assert!(apply(&conn, &migrations, 2).is_err());
        assert_eq!(version(&conn)?, 1);
        assert!(conn.is_autocommit(), "no transaction left open");
        Ok(())
    }
}
