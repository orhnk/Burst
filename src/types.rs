use std::error::Error as StdErr;

use crate::data::Data;

pub type Error = Box<dyn StdErr + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub type PartialContext<'a> = poise::PartialContext<'a, Data, Error>;

pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, Error>;

pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;
