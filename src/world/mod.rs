pub mod builder;

use crate::phrases::*;
use ron;
use serde::Deserialize;
use std::collections::HashMap;

/// Controlling entity.
#[derive(Deserialize)]
pub enum Contr {
    Cpu,
    Player,
}

/// Cook entity.
#[derive(Deserialize)]
pub struct Cook {
    pub name: String,
}

pub type CookKey = usize;

/// Judge entity.
#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Judge {
    pub name: String,
    pub phrases: Phrases,
}

pub type JudgeKey = usize;

/// The score given to cooks from judges.
pub type Score = u32;

/// Entire state of the game world.
#[derive(Deserialize)]
pub struct World {
    pub cooks: Vec<Cook>,
    pub judges: Vec<Judge>,
    pub ranking: HashMap<CookKey, Score>,
    pub default_phrases: Phrases,
}

impl World {
    pub fn new() -> World {
        World {
            cooks: vec![],
            judges: vec![],
            ranking: HashMap::new(),
            default_phrases: Phrases::default(),
        }
    }
}
