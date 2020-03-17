use crate::world::*;
use lazy_static::*;
use regex::Regex;
use serde::Deserialize;

/// All the prhases a judge might use to judge a zuppa.
#[derive(Default, Debug, PartialEq, Deserialize)]
pub struct Phrases {
    phrases: Vec<(Score, String)>,
}

impl Phrases {
    /// Parse a special format string into a `Phrases` struct.
    pub fn parse(from: &str) -> Self {
        // Regex to match single phrase.
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"(\d*):\s*([\.\-!'",;\[\]\(\){}\sa-zA-Z0-9_]*)$"#).unwrap();
        }

        Phrases {
            phrases: from
                .lines()
                .map(|phrase| {
                    let caps = RE.captures(phrase).unwrap_or_else(|| panic!("Malformed phrase: {}", phrase));

                    // Info to construct phrase is extracted from captures of regex match.
                    let score_upper_bound_exclusive = caps
                        .get(1)
                        .expect("Invalid upper range match in phrase config file")
                        .as_str()
                        .parse()
                        .expect("Malformed phrase upper score range specifier");
                    let phrase_template = caps
                        .get(2)
                        .expect("Invalid phrase template match in phrase config file")
                        .as_str()
                        .into();

                    (score_upper_bound_exclusive, phrase_template)
                })
                .collect(),
        }
    }

    /// Use world info to output and deparameterize the correct phrase.
    pub fn generate(&self, world: &World, judge_k: JudgeKey, cook_k: CookKey, score: Score) -> String {
        for phrase in &self.phrases {
            if score < phrase.0 {
                return Self::generate_impl(&phrase.1, world, judge_k, cook_k, score);
            }
        }

        // Fall back on default phrase.
        format!("{} is extatic...", world.judges[judge_k].name)
    }

    /// Actual phrase deparameterization heavy lifting.
    #[allow(unused_variables)]
    pub fn generate_impl(phrase: &str, world: &World, judge_k: JudgeKey, cook_k: CookKey, score: Score) -> String {
        // TODO: actually do this...
        phrase.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phrases_parse_single_phrase() {
        assert_eq!(
            Phrases::parse("0: I like this"),
            Phrases {
                phrases: vec! {
                    (0, "I like this".into()),
                }
            }
        )
    }
}
