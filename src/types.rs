use std::error::Error as StdErr;

pub type Error = Box<dyn StdErr + Send + Sync>;
