use crate::noun::Noun;
use crate::world::*;
use lazy_static::*;
use regex::Regex;
use serde::Deserialize;

/// All the info from a zuppa interaction needed to deparameterize a phrase.
#[derive(Clone)]
pub struct Context {
    pub judge_k: JudgeKey,
    pub cook_k: CookKey,
    pub score: Score,
}

/// TODO: WIP.
struct PropMods;

impl PropMods {
    /// TODO: WIP.
    fn apply(self, prop: &str) -> String {
        prop.into()
    }
}

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
            static ref RE: Regex =
                Regex::new(r#"(\d*):\s*([\.\-!'",;\[\]\(\){}\sa-zA-Z0-9_]*)$"#).unwrap();
        }

        Phrases {
            phrases: from
                .lines()
                .map(|phrase| {
                    let caps = RE
                        .captures(phrase)
                        .unwrap_or_else(|| panic!("Malformed phrase: {}", phrase));

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
    pub fn generate(&self, world: &World, ctx: Context) -> String {
        for phrase in &self.phrases {
            if ctx.score < phrase.0 {
                return Self::generate_impl(&phrase.1, world, ctx);
            }
        }

        // Fall back on default phrase.
        format!("{} is extatic...", world.judges[ctx.judge_k].name)
    }

    /// Actual phrase deparameterization heavy lifting.
    /// TODO: Clean this shit...
    pub fn generate_impl(phrase: &str, world: &World, context: Context) -> String {
        // TODO: Use prelude thing.
        use wc::parse::*;

        // Parser to retreive properties from a zuppa interation.
        let inter_prop = |ctx: Context| {
            move |input: String| {
                let cook = &world.cooks[ctx.cook_k];
                let judge = &world.judges[ctx.judge_k];

                // TODO: Check if clone can be avoided.
                match input.as_str() {
                    "judge" => judge.name.clone(),
                    "cook" => cook.name.clone(),
                    "pron" => cook.pronoun().into(),
                    "poss" => cook.poss_pronoun().into(),
                    _ => panic!("Malformed prop in phrase escape sequence: {}", input),
                }
            }
        };

        // TODO: Make this do something.
        let inter_mods = |_ctx: Context| |_input| PropMods;

        // Parser that handles an escape sequence.
        let escape_seq = |ctx: Context| {
            let inner = identifier
                .map(inter_prop(ctx.clone()))
                .and_then(move |prop| {
                    opt(right(literal(":"), identifier.map(inter_mods(ctx.clone())))).map(
                        move |prop_mods| match prop_mods {
                            Some(prop_mods) => prop_mods.apply(&prop),
                            None => prop.clone(),
                        },
                    )
                });

            sorround(literal("{"), inner, literal("}"))
        };

        // The actual parser.
        let phrase_parser = zero_or_more(pair(
            either(
                // Normal word or phrase part.
                one_or_more(
                    one_or_more(any_char.pred(|&c| c != '{' && !c.is_whitespace()))
                        .map(|s| s.into_iter().collect::<String>()),
                )
                .map(|s| s.into_iter().collect::<String>()),
                // Escape sequence.
                escape_seq(context),
            ),
            space0(),
        ))
        .map(|out| {
            out.into_iter()
                .map(|(word, wspace)| format!("{}{}", word, wspace.into_iter().collect::<String>()))
        })
        .map(|s| {
            #[allow(clippy::all)] // TODO: Report false positive.
            s.into_iter().collect()
        });

        phrase_parser.parse(phrase).expect("Malformed phrase").1
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
