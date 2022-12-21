mod ping;

use poise::Command;

use crate::{
    data::Data,
    types::Error,
};

pub fn commands() -> Vec<Command<Data, Error>> {
    vec![ping::ping()]
}
