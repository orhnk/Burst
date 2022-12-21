use std::error::Error as StdError;

pub type Error = Box<dyn StdError + Send + Sync>;
