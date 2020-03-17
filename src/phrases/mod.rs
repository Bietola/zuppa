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
