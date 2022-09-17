use super::Difference;
use crate::chart::RuntimeNote;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    thread::current,
};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct JudgeWindow(pub i32);
pub static JUDGE: [JudgeWindow; 8] = [
    JudgeWindow(-118),
    JudgeWindow(-85),
    JudgeWindow(-51),
    JudgeWindow(-18),
    JudgeWindow(17),
    JudgeWindow(50),
    JudgeWindow(84),
    JudgeWindow(117),
];

pub type Judgement = HashMap<RuntimeNote, JudgeWindow>;
pub type Boo = HashSet<u32>;

#[derive(Debug, Clone)]
pub struct Judge {
    pub judgements: Judgement,
    pub boos: Boo,
}

impl Judge {
    /// Creates a new [`Judge`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            judgements: HashMap::default(),
            boos: HashSet::default(),
        }
    }

    /// Try to calculate a judge window for a note.
    ///
    /// # Errors
    /// Returns [`Err(None)`] if the note was already judged.
    pub fn judge(
        &mut self,
        current_timestamp: u32,
        closest_note: &RuntimeNote,
    ) -> anyhow::Result<Option<JudgeWindow>> {
        if !self.judgements.contains_key(closest_note) {
            let diff = closest_note.timestamp.diff(&(current_timestamp));
            let signed_offset: i8 = if let Ok(small_offset) = diff.try_into() {
                let negative = closest_note.timestamp < current_timestamp;

                let result = if negative {
                    small_offset * -1
                } else {
                    small_offset
                };
                result
            } else {
                i8::MAX
            };

            let judge = calculate_judge_window(signed_offset);

            if let Some(some_judge) = judge {
                let local_note = closest_note.clone();
                log::debug!(
                    "Judgement: {:?} on note: {:?} at ",
                    some_judge,
                    local_note.timestamp,
                );
                self.judgements.insert(local_note, some_judge);
            } else {
                self.boos.insert(current_timestamp);
                log::debug!("Boo at: {:?}", current_timestamp);
            }

            Ok(judge)
        } else {
            log::error!("Already judged: {:?}", closest_note);
            Err(anyhow::anyhow!("Already judged"))
        }
    }
}

fn calculate_judge_window(hit_offset: i8) -> Option<JudgeWindow> {
    let mut last_judge = None;
    for judge in JUDGE {
        if i32::from(hit_offset) > judge.0 {
            last_judge.replace(judge);
        }
    }
    last_judge
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judgement_to_window() {
        let offset = -119i8;
        assert_eq!(calculate_judge_window(offset), None);

        let offset = -90i8;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[0]));

        let offset = -70i8;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[1]));

        let offset = -50i8;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[2]));

        let offset = -14i8;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[3]));

        let offset = 18i8;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[4]));

        let offset = 51i8;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[5]));

        let offset = 90i8;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[6]));

        let offset = 118i8;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[7]));
    }
}
