use crate::world::*;
use std::collections::HashMap;
use serde::Deserialize;

/// All the prhases a judge might use to judge a zuppa.
#[derive(Default, Deserialize)]
pub struct Phrases {
    phrases: HashMap<Score, Phrase>,
}

impl Phrases {
    /// Parse a special format string into a `Phrases` struct.
    pub fn parse(from: &str) -> Self {
        unimplemented!()
    }
}

/// Single judge phrase.
#[derive(Deserialize)]
pub struct Phrase {
    contents: String,
}

impl Phrase {
    pub fn gen(&self, world: &World, cook: CookKey, judge: JudgeKey) -> String {
        unimplemented!()
    }
}
