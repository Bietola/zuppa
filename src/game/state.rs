use crate::game::world::GameWorld;
use crate::view::View;

/// A GameState defines what happens at every game iteration through the `step` member function.
trait GameState<G, V>
{
    fn step(&mut self, world: &mut GameWorld, view: &mut V) -> G;
}

/// Game is introduced bombastically.
struct Intro;

impl<G, V> GameState<G, V> for Intro
where G: GameState<G, V>,
      V: View,
{
    fn step(&mut self, world: &mut GameWorld, view: &mut V) -> G {
        println!("test");

        Intro
    }
}
