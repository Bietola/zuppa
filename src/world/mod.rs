pub mod builder;
pub mod zuppa;

use crate::phrases::*;
use itertools::Itertools;
use ron;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use zuppa::*;
use std::iter::FromIterator;

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
    pub contr: Contr,
}

pub type CookKey = usize;

/// Judge entity.
#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Judge {
    pub name: String,
    pub phrases: Phrases,
}

impl Judge {
    /// Is completely random for now and does not depend on anything... maybe in the future is will
    /// be more sophisticated.
    pub fn judge_zuppa(&self, _world: &World, _zuppa: &Zuppa) -> Score {
        // [0, 100)
        use rand::Rng;
        rand::thread_rng().gen_range(0, 100)
    }
}

pub type JudgeKey = usize;

/// The score given to cooks from judges.
pub type Score = u32;

/// The player ranking
#[derive(Clone, Default, Deserialize)]
pub struct Ranking {
    pub ranking: HashMap<CookKey, Score>,
}

impl Ranking {
    fn empty() -> Self {
        Default::default()
    }

    /// For showing the ranking on the screen.
    pub fn to_pretty_string(&self, world: &World) -> String {
        self.ranking
            .iter()
            // Sort by scores
            .sorted_by(|lhs, rhs| Ord::cmp(lhs.1, rhs.1))
            .map(|(&cook_k, &score)| format!("{}: {}", score, world.cooks[cook_k].name))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl FromIterator<(CookKey, Score)> for Ranking {
    fn from_iter<I: IntoIterator<Item = (CookKey, Score)>>(iter: I) -> Self {
        Ranking {
            ranking: iter.into_iter().collect(),
        }
    }
}

/// Entire state of the game world.
#[derive(Deserialize)]
pub struct World {
    pub cooks: Vec<Cook>,
    pub judges: Vec<Judge>,
    pub ranking: Ranking,
    pub default_phrases: Phrases,
}

impl World {
    pub fn new() -> World {
        World {
            cooks: vec![],
            judges: vec![],
            ranking: Ranking::empty(),
            default_phrases: Phrases::default(),
        }
    }

    /// Pick a judge at random
    pub fn pick_random_judge(&self) -> JudgeKey {
        use rand::Rng;
        rand::thread_rng().gen_range(0, self.judges.len())
    }
}
