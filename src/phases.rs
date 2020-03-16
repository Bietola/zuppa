use crate::world::World;
use crate::view::base::*;
use crate::msgln;

/// Game is introduced bombastically.
pub fn intro(world: &mut World, v: &mut impl View) {
    msgln!(v, "Welcome to zuppa!");

    msgln!(v, "The cooks:");
    for cook in &world.cooks {
        msgln!(v, "{}", cook.name);
    }

    msgln!(v, "The judges:");
    for judge in &world.judges {
        msgln!(v, "{}", judge.name);
    }

    unimplemented!()
    // TODO: slaughter(world, v);
}

// /// Phase where all the gameplay takes place, for now.
// fn slaughter(world: &mut GameWorld, view: &mut impl View) {
//     msgln!(v, "Let the slaughter begin!");

//     // Keep old ranking for later comparison.
//     let old_ranking = world.ranking.clone();

//     // Every cooks is challenged to cook a zuppa and the randking is updated with their new score.
//     world.ranking = world.cooks.iter()
//         .enumerate()
//         .map(|(i, _)| i)
//         .map(|cook_i| {
//             // Judge is picked to taste the contendent's zuppa.
//             let judge_i = world.pick_random_judge();

//             let zuppa = cook_zuppa_interaction(v, world, cook_i);
//             let score = world.judges[judge_i].judge_zuppa(v, world, zuppa);

//             (cook_i, score)
//         })
//         .collect();

//     // Show new raking compared to old one.
//     show_raking(v, old_ranking, world.raking);

//     // Cook with the lowest raking is eliminated.
//     println!("WIP: Elimination...");
//     slaughter(world, v);
// }

//         // msgln!(v, "{} is cooking for {}", w.cooks[cook_i].name, judge.name);
