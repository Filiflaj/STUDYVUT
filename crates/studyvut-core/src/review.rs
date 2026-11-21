//! Spaced repetition with Leitner boxes.
//!
//! A question enters box 1 on its first answer. A right answer moves it one
//! box up (at most [`BOXES`]), a wrong one back to box 1. It is due again
//! after the box's interval in days: 1, 2, 4, 8, 16. Days are the user's
//! local days ([`crate::local_day`]), so a question answered in the evening
//! is due the next morning, not the next evening.

use serde::Serialize;

/// The highest box.
pub const BOXES: u8 = 5;

/// Days until a question in box `n` (1-based) is due again.
pub const INTERVAL_DAYS: [i64; BOXES as usize] = [1, 2, 4, 8, 16];

/// Where a question stands after an answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Schedule {
    pub r#box: u8,
    /// The local day from which it is due.
    pub due_day: i64,
}

/// The schedule after answering on local day `today`; `previous` is the
/// box before (None for a first answer).
pub fn next(previous: Option<u8>, correct: bool, today: i64) -> Schedule {
    let b = if correct {
        previous.map_or(1, |b| b.saturating_add(1)).clamp(1, BOXES)
    } else {
        1
    };
    Schedule {
        r#box: b,
        due_day: today + INTERVAL_DAYS[usize::from(b - 1)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_answers_climb_and_wrong_ones_start_over() {
        let mut s = next(None, true, 100);
        assert_eq!(
            s,
            Schedule {
                r#box: 1,
                due_day: 101
            }
        );
        for (b, due) in [(2, 102), (3, 104), (4, 108), (5, 116), (5, 116)] {
            s = next(Some(s.r#box), true, 100);
            assert_eq!(
                s,
                Schedule {
                    r#box: b,
                    due_day: due
                }
            );
        }
        assert_eq!(
            next(Some(5), false, 100),
            Schedule {
                r#box: 1,
                due_day: 101
            }
        );
        assert_eq!(
            next(None, false, 100),
            Schedule {
                r#box: 1,
                due_day: 101
            }
        );
        assert_eq!(
            next(Some(0), true, 0).r#box,
            1,
            "an invalid box is repaired"
        );
        assert_eq!(next(Some(200), true, 0).r#box, BOXES);
    }
}
