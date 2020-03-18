pub mod builder;
pub mod zuppa;

use crate::phrases::*;
use itertools::Itertools;
use ron;
use serde::Deserialize;
use std::collections::HashMap;
use zuppa::*;
use std::iter::FromIterator;
use crate::noun::{Gender, Noun};

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
    pub gender: Gender,
}

impl Noun for Cook {
    fn get_gender(&self) -> Gender {
        self.gender
    }
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
    pub data: HashMap<CookKey, Score>,
}

impl Ranking {
    fn empty() -> Self {
        Default::default()
    }

    /// For showing the ranking on the screen.
    pub fn to_pretty_string(&self, world: &World) -> String {
        self.data
            .iter()
            // Sort by scores
            .sorted_by(|lhs, rhs| Ord::cmp(rhs.1, lhs.1))
            .map(|(&cook_k, &score)| format!("{}: {}", score, world.cooks[cook_k].name))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl FromIterator<(CookKey, Score)> for Ranking {
    fn from_iter<I: IntoIterator<Item = (CookKey, Score)>>(iter: I) -> Self {
        Ranking {
            data: iter.into_iter().collect(),
        }
    }
}

/// Entire state of the game world.
#[derive(Deserialize)]
pub struct World {
    pub cooks: Vec<Cook>,
    pub cooks_in_game: Vec<CookKey>,
    pub judges: Vec<Judge>,
    pub ranking: Ranking,
    pub default_phrases: Phrases,
}

impl World {
    pub fn new() -> World {
        World {
            cooks: vec![],
            cooks_in_game: vec![],
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

    pub fn eliminate_cook(&mut self, cook_k: CookKey) {
        self.cooks_in_game.retain(|&ele| ele != cook_k);
    }
}
