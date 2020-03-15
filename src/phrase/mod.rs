use nom;
use specs::prelude::*;

pub struct Phrase {
    contents: String,
}

impl Phrase {
    pub fn parse(&self, world: &World, cook: Entity, judge: Entity) -> String {
        unimplemented!()
    }
}
