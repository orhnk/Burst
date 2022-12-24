mod misc;
mod owner;

use crate::types::CommandVec;

pub fn commands() -> CommandVec {
    vec![misc::commands(), owner::commands()]
        .into_iter()
        .flatten()
        .collect()
}
