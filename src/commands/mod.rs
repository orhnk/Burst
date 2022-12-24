mod misc;

use crate::types::CommandVec;

pub fn commands() -> CommandVec {
    vec![misc::commands()].into_iter().flatten().collect()
}
