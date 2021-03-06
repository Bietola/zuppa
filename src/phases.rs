use crate::phrases;
use crate::view::base::*;
use crate::world::{zuppa::*, *};
use crate::{msg, msgln};
use single::Single;
use text_io::read;

// Commands used in zuppa cooking interaction.
const EXIT_COMMAND: &str = "exit";

/// Game is introduced bombastically.
pub fn intro(world: &mut World, v: &mut impl View) {
    msgln!(v, "Welcome to zuppa!");
    msgln!(v);

    msgln!(v, "The cooks:");
    for cook in &world.cooks {
        msgln!(v, "{}", cook.name);
    }
    msgln!(v);

    msgln!(v, "The judges:");
    for judge in &world.judges {
        msgln!(v, "{}", judge.name);
    }
    msgln!(v);

    slaughter(world, v);
}

/// Phase where all the gameplay takes place, for now.
fn slaughter(world: &mut World, v: &mut impl View) {
    msgln!(v, "Let the slaughter begin!");

    // Keep old ranking for later comparison.
    // TODO: use this for showing ranking diff
    let _old_ranking = world.ranking.clone();

    // Every cooks is challenged to cook a zuppa and the randking is updated with their new score.
    world.ranking = world
        .cooks_in_game
        .iter()
        .map(|&cook_i| {
            // Judge is picked to taste the contendent's zuppa.
            let judge_i = world.pick_random_judge();

            let zuppa = cook_interaction(v, world, judge_i, cook_i);
            let score = judge_interaction(v, world, judge_i, zuppa);

            msgln!(v);

            (
                cook_i,
                world.ranking.data.get(&cook_i).unwrap_or(&0) + score,
            )
        })
        .collect::<Ranking>();

    // Show new raking compared to old one.
    msgln!(v, "{}", world.ranking.to_pretty_string(world));
    msgln!(v);

    // Cook with the lowest score is eliminated.
    let eliminee = *world
        .ranking
        .data
        .iter()
        .min_by_key(|(_, &score)| score)
        .expect("Could not find cook with min score to eliminate")
        .0;
    world.eliminate_cook(eliminee);
    msgln!(v, "{} was eliminated...", world.cooks[eliminee].name);
    msgln!(v);

    // Game is over if only one cook is left.
    if world.cooks_in_game.len() == 1 {
        end(world, v);

    // Go on otherwise.
    } else {
        slaughter(world, v);
    }
}

/// Make a cook cook a zuppa for a particular judge.
fn cook_interaction(v: &mut impl View, world: &World, judge_k: JudgeKey, cook_k: CookKey) -> Zuppa {
    let cook = &world.cooks[cook_k];
    let judge = &world.judges[judge_k];

    msgln!(v, "{} is cooking for {}!", cook.name, judge.name);

    match cook.contr {
        Contr::Cpu => {
            // TODO: use more advanced CPUs.
            Zuppa {
                author: cook_k,
                ingredients: vec!["silicon".into()],
            }
        }
        Contr::Player => {
            // Zuppa accumulator.
            let mut zuppa = Zuppa {
                author: cook_k,
                ..Zuppa::default()
            };

            loop {
                msg!(v, "> ");
                let command: String = read!("{}\n");

                match command.as_str() {
                    EXIT_COMMAND => return zuppa,

                    // Any non-special keyword is treated as an ingredient.
                    ingredient => zuppa.ingredients.push(ingredient.into()),
                }
            }
        }
    }
}

/// Make a judge judge a particular zuppa from a particular cook.
fn judge_interaction(v: &mut impl View, world: &World, judge_k: JudgeKey, zuppa: Zuppa) -> Score {
    let judge = &world.judges[judge_k];

    // Judgement score is calculated and used for extracting the right judgement catchphrase.
    // Said phrase (which is stored generically) needs to be parameterized with the cook's and the judge's info.
    let score = judge.judge_zuppa(world, &zuppa);
    let ctx = phrases::Context {
        judge_k,
        cook_k: zuppa.author,
        score,
    };
    let judgement = judge.phrases.generate(world, ctx);

    msgln!(v, "{} [{}]", judgement, score);

    score
}

fn end(world: &World, v: &mut impl View) {
    let winner = &world.cooks[*world
        .cooks_in_game
        .iter()
        .single()
        .expect("Wrong number of winners at game end")];

    msgln!(v, "The winner of this Zuppa tournament is: {}", winner.name);

    // No state transition means program termination.
}
