use std::error::Error as StdErr;

use poise::Command;

use crate::data::Data;

pub type Error = Box<dyn StdErr + Send + Sync>; // Why There is a Sync? Errors do not have to be mutable?

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub type PartialContext<'a> = poise::PartialContext<'a, Data, Error>;

pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, Error>;

pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;

pub type MaybeError = Result<(), Error>;

pub type CommandVec = Vec<Command<Data, Error>>;
