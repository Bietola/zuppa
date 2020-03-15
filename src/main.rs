mod phrase;
mod game;
mod view;

use view::{
    base::*,
    simple_term::*,
};

fn main() {
    // Initialize view.
    let mut view = SimpleTermView;

    // Initialize game.
    // let game = GameBuilder::new()
    //     .load_players("assets/players.cfg")
    //     .load_phrases("assets/phrases")
    //     .build();

    // Game loop.
    // while !game.is_over() {
    //     game_state.step();
    // }
}
