//! STUDYVUT core: everything that is not the app's window.
//!
//! - [`content`]: the study material (subjects, chapters, quizzes) as read
//!   from `content/`, and the checker behind `studyvut-cli check`;
//! - [`quiz`]: evaluating an answer and choosing the questions of a set;
//! - [`review`]: spaced repetition (Leitner boxes);
//! - [`store`]: the user's progress in SQLite (feature `store`).
//!
//! No Tauri and no UI here, so the desktop app and the CLI share it.

pub mod content;
pub mod quiz;
pub mod review;
#[cfg(feature = "store")]
pub mod store;

/// Milliseconds per day.
pub const DAY_MS: i64 = 86_400_000;

/// The user's local day (days since 1970-01-01 in their time zone) of a Unix
/// time in ms. `offset_min` is the zone's offset east of UTC in minutes
/// (the browser's `-getTimezoneOffset()`).
pub fn local_day(ts_ms: i64, offset_min: i32) -> i64 {
    (ts_ms + i64::from(offset_min) * 60_000).div_euclid(DAY_MS)
}

/// Now as Unix time in ms.
pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_day_follows_the_time_zone() {
        // 2026-11-21 23:30 UTC is already the 22nd in Prague (UTC+1).
        let ts = 1_795_303_800_000;
        assert_eq!(local_day(ts, 0) + 1, local_day(ts, 60));
        assert_eq!(local_day(0, 0), 0);
        assert_eq!(local_day(-1, 0), -1, "before 1970 rounds down");
    }
}
