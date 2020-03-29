use super::world::{zuppa::*, *};
use crate::phrases;
use single::Single;
use std::collections::HashMap;
use text_io::read;
use super::netmsg;
use super::netutils::{Sender, Receiver};
use futures::sink::SinkExt;

// Commands used in zuppa cooking interaction.
const EXIT_COMMAND: &str = "exit";

/****************************/
/* Net connection utilities */
/****************************/

// All connections indexed by their character keys for quick retrieval in game.
type NetMsg = netmsg::NetMsg<String>;
type Connections = HashMap<ActorKey, Sender<NetMsg>>;

// Macros for convient simple message broadcasting and sending.
macro_rules! msg {
    ($connections:expr, $fmt_str:expr, $( $fmt_arg:expr ),+) => {
        $connections.values_mut().map(|c| c.send(NetMsg::Msg(format!($fmt_str, $($fmt_arg,)*))));
    };

    ($connections:expr, $fmt_str:expr) => {
        msg!($connections, "{}", $fmt_str);
    };
}

macro_rules! msgln {
    ($connections:expr, $fmt_str:expr, $( $fmt_arg:expr ),+) => {
        msg!($connections, concat!(stringify!($fmt_str), "\n"), $( $fmt_arg ),*);
    };

    ($connections:expr, $fmt_str:expr) => {
        msgln!($connections, "{}", $fmt_str);
    };

    ($connections:expr) => {
        msgln!($connections, "");
    };
}

/***************/
/* Game phases */
/***************/

/// Game is introduced bombastically.
pub fn intro(world: &mut World, cs: &mut Connections) {
    msg!(cs, "Welcome to zuppa!");
    msgln!(cs);

    msgln!(cs, "The cooks:");
    for cook in &world.cooks {
        msgln!(cs, "{}", cook.name);
    }
    msgln!(cs);

    msgln!(cs, "The judges:");
    for judge in &world.judges {
        msgln!(cs, "{}", judge.name);
    }
    msgln!(cs);

    slaughter(world, cs);
}

/// Phase where all the gameplay takes place, for now.
fn slaughter(world: &mut World, cs: &mut Connections) {
    msgln!(cs, "Let the slaughter begin!");

    // Keep old ranking for later comparison.
    // TODO: use this for showing ranking diff
    let _old_ranking = world.ranking.clone();

    // Ecs cooks is challenged to cook a zuppa and the randking is updated with their new score.
    world.ranking = world
        .cooks_in_game
        .iter()
        .map(|&cook_i| {
            // Judge is picked to taste the contendent's zuppa.
            let judge_i = world.pick_random_judge();

            let zuppa = cook_interaction(cs, world, judge_i, cook_i);
            let score = judge_interaction(cs, world, judge_i, zuppa);

            msgln!(cs);

            (
                cook_i,
                world.ranking.data.get(&cook_i).unwrap_or(&0) + score,
            )
        })
        .collect::<Ranking>();

    // Show new raking compared to old one.
    msgln!(cs, "{}", world.ranking.to_pretty_string(world));
    msgln!(cs);

    // Cook with the lowest score is eliminated.
    let eliminee = *world
        .ranking
        .data
        .iter()
        .min_by_key(|(_, &score)| score)
        .expect("Could not find cook with min score to eliminate")
        .0;
    world.eliminate_cook(eliminee);
    msgln!(cs, "{} was eliminated...", world.cooks[eliminee].name);
    msgln!(cs);

    // Game is ocs if only one cook is left.
    if world.cooks_in_game.len() == 1 {
        end(world, cs);

    // Go on otherwise.
    } else {
        slaughter(world, cs);
    }
}

/// Make a cook cook a zuppa for a particular judge.
fn cook_interaction(cs: &mut Connections, world: &World, judge_k: JudgeKey, cook_k: CookKey) -> Zuppa {
    let cook = &world.cooks[cook_k];
    let judge = &world.judges[judge_k];

    msgln!(cs, "{} is cooking for {}!", cook.name, judge.name);

    match cook.contr {
        Contr::Cpu => {
            // TODO: use more adcs CPUs.
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
                msg!(cs, "> ");
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
fn judge_interaction(cs: &mut Connections, world: &World, judge_k: JudgeKey, zuppa: Zuppa) -> Score {
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

    msgln!(cs, "{} [{}]", judgement, score);

    score
}

fn end(world: &World, cs: &mut Connections) {
    let winner = &world.cooks[*world
        .cooks_in_game
        .iter()
        .single()
        .expect("Wrong number of winners at game end")];

    msgln!(
        cs,
        "The winner of this Zuppa tournament is: {}",
        winner.name
    );

    // No state transition means program termination.
}
