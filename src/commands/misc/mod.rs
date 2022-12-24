use crate::types::CommandVec;

mod help;
mod ping;

pub fn commands() -> CommandVec {
    vec![help::help(), ping::ping()]
}
