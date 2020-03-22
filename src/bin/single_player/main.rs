use world::*;
use view::simple_term::*;

fn main() {
    // Initialize view.
    let mut view = SimpleTermView;

    // Initialize game.
    use world::builder::Builder;
    let mut world = world::builder::Builder::new()
        .with_players_file("assets/players.ron")
        .with_phrases_dir("assets/phrases")
        .map(Builder::build)
        .expect("Error while building world.");

    // Game starts.
    phases::intro(&mut world, &mut view);
}
