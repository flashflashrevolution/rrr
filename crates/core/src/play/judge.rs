use super::Difference;
use crate::chart::RuntimeNote;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    ops::Neg,
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
        if self.judgements.contains_key(closest_note) {
            return Err(anyhow::anyhow!("Already judged"));
        }

        log::info!(
            "Timestamp of Strike: {:?} || Timestamp of Note: {:?}",
            current_timestamp,
            closest_note.timestamp
        );

        let diff: u32 = closest_note.timestamp.abs_diff(current_timestamp);

        let signed_offset = if let Ok(small_offset) = i32::try_from(diff) {
            let negative = closest_note.timestamp < current_timestamp;

            if negative {
                small_offset.neg()
            } else {
                small_offset
            }
        } else {
            i32::MAX
        };

        log::info!("Difference: {:?}", signed_offset);

        let judge = calculate_judge_window(signed_offset);
        log::info!("Judgement: {:?}", judge);

        if let Some(some_judge) = judge {
            let local_note = closest_note.clone();
            self.judgements.insert(local_note, some_judge);
        } else {
            self.boos.insert(current_timestamp);
        }

        Ok(judge)
    }
}

fn calculate_judge_window(hit_offset: i32) -> Option<JudgeWindow> {
    let mut last_judge = None;
    for judge in JUDGE {
        if hit_offset > judge.0 {
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
        let offset = 4;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[3]));

        let offset = -119;
        assert_eq!(calculate_judge_window(offset), None);

        let offset = -90;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[0]));

        let offset = -70;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[1]));

        let offset = -50;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[2]));

        let offset = -14;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[3]));

        let offset = 18;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[4]));

        let offset = 51;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[5]));

        let offset = 90;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[6]));

        let offset = 118;
        assert_eq!(calculate_judge_window(offset), Some(JUDGE[7]));
    }
}
