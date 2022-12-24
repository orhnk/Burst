use crate::types::CommandVec;

mod sync_app_commands;

pub fn commands() -> CommandVec {
    vec![sync_app_commands::sync_app_commands()]
}
