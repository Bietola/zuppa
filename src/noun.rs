use serde::{Deserialize, Serialize};

/// Simple gender enum.
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Gender {
    Neuter,
    Female,
    Male,
}

impl Default for Gender {
    fn default() -> Self {
        Self::Neuter
    }
}

/// Noun here is intended in the grammatical sense.
pub trait Noun {
    fn get_gender(&self) -> Gender;

    fn pronoun(&self) -> &'static str {
        match self.get_gender() {
            Gender::Male => "he",
            Gender::Female => "she",
            Gender::Neuter => "it",
        }
    }

    fn poss_pronoun(&self) -> &'static str {
        match self.get_gender() {
            Gender::Male => "his",
            Gender::Female => "her",
            Gender::Neuter => "its",
        }
    }
}
