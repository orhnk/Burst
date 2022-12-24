mod help;
mod ping;
use poise::Command;

use crate::{
    data::Data,
    types::Error,
};

pub fn commands() -> Vec<Command<Data, Error>> {
    vec![help::help(), ping::ping()]
}
